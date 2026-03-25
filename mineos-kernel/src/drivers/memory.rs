// =============================================================================
// MineOS - Memory Management (Rust)
// =============================================================================
// Handles physical and virtual memory, page frame allocation,
// and the kernel heap allocator.
// =============================================================================

use bootloader::BootInfo;
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{
    FrameAllocator, Mapper, OffsetPageTable, Page, PageTable,
    PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};
use linked_list_allocator::LockedHeap;

/// Kernel heap configuration
pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 4 * 1024 * 1024; // 4 MB heap

/// Global allocator
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Initialize memory subsystem
pub fn init(boot_info: &'static BootInfo) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    
    let mut mapper = unsafe { init_page_table(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    // Allocate heap pages
    allocate_heap(&mut mapper, &mut frame_allocator)
        .expect("Heap allocation failed");
    
    // Initialize the heap allocator
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}

/// Initialize the page table mapper
unsafe fn init_page_table(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Get the active Level 4 page table
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;
    
    let (level_4_frame, _) = Cr3::read();
    let phys = level_4_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    
    &mut *page_table_ptr
}

/// Allocate pages for the kernel heap
fn allocate_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), &'static str> {
    use x86_64::structures::paging::PageTableFlags as Flags;
    
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or("Failed to allocate frame for heap")?;
        let flags = Flags::PRESENT | Flags::WRITABLE;
        unsafe {
            mapper
                .map_to(page, frame, flags, frame_allocator)
                .map_err(|_| "Failed to map heap page")?
                .flush();
        }
    }
    
    Ok(())
}

/// Frame allocator that uses the bootloader's memory map
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a new frame allocator from the memory map
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Get an iterator over usable memory frames
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
