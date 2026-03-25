from fastapi import FastAPI, APIRouter, HTTPException
from dotenv import load_dotenv
from starlette.middleware.cors import CORSMiddleware
from motor.motor_asyncio import AsyncIOMotorClient
import os
import logging
from pathlib import Path
from pydantic import BaseModel, Field, ConfigDict
from typing import List, Optional
import uuid
from datetime import datetime, timezone

ROOT_DIR = Path(__file__).parent
load_dotenv(ROOT_DIR / '.env')

mongo_url = os.environ['MONGO_URL']
client = AsyncIOMotorClient(mongo_url)
db = client[os.environ['DB_NAME']]

app = FastAPI()
api_router = APIRouter(prefix="/api")

# Models
class FileItem(BaseModel):
    model_config = ConfigDict(extra="ignore")
    id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    name: str
    path: str
    type: str  # "file" or "folder"
    content: str = ""
    parent_path: str = "/"
    size: int = 0
    created_at: str = Field(default_factory=lambda: datetime.now(timezone.utc).isoformat())
    updated_at: str = Field(default_factory=lambda: datetime.now(timezone.utc).isoformat())

class FileCreate(BaseModel):
    name: str
    path: str
    type: str
    content: str = ""
    parent_path: str = "/"

class FileUpdate(BaseModel):
    content: Optional[str] = None
    name: Optional[str] = None

class NoteItem(BaseModel):
    model_config = ConfigDict(extra="ignore")
    id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    title: str
    content: str = ""
    created_at: str = Field(default_factory=lambda: datetime.now(timezone.utc).isoformat())
    updated_at: str = Field(default_factory=lambda: datetime.now(timezone.utc).isoformat())

class NoteCreate(BaseModel):
    title: str
    content: str = ""

# File System APIs
@api_router.get("/")
async def root():
    return {"message": "MineOS API"}

@api_router.get("/files", response_model=List[FileItem])
async def get_files(parent_path: str = "/"):
    files = await db.files.find({"parent_path": parent_path}, {"_id": 0}).to_list(1000)
    return files

@api_router.get("/files/all", response_model=List[FileItem])
async def get_all_files():
    files = await db.files.find({}, {"_id": 0}).to_list(5000)
    return files

@api_router.post("/files", response_model=FileItem, status_code=201)
async def create_file(input_data: FileCreate):
    existing = await db.files.find_one({"path": input_data.path}, {"_id": 0})
    if existing:
        raise HTTPException(status_code=400, detail="File already exists")
    file_obj = FileItem(**input_data.model_dump())
    file_obj.size = len(input_data.content)
    doc = file_obj.model_dump()
    await db.files.insert_one(doc)
    return file_obj

@api_router.put("/files/{file_id}", response_model=FileItem)
async def update_file(file_id: str, input_data: FileUpdate):
    update_dict = {}
    if input_data.content is not None:
        update_dict["content"] = input_data.content
        update_dict["size"] = len(input_data.content)
    if input_data.name is not None:
        update_dict["name"] = input_data.name
    update_dict["updated_at"] = datetime.now(timezone.utc).isoformat()
    
    result = await db.files.find_one_and_update(
        {"id": file_id},
        {"$set": update_dict},
        return_document=True,
        projection={"_id": 0}
    )
    if not result:
        raise HTTPException(status_code=404, detail="File not found")
    return result

@api_router.delete("/files/{file_id}")
async def delete_file(file_id: str):
    result = await db.files.delete_one({"id": file_id})
    if result.deleted_count == 0:
        raise HTTPException(status_code=404, detail="File not found")
    return {"status": "deleted"}

# Notes APIs
@api_router.get("/notes", response_model=List[NoteItem])
async def get_notes():
    notes = await db.notes.find({}, {"_id": 0}).to_list(1000)
    return notes

@api_router.post("/notes", response_model=NoteItem, status_code=201)
async def create_note(input_data: NoteCreate):
    note_obj = NoteItem(**input_data.model_dump())
    doc = note_obj.model_dump()
    await db.notes.insert_one(doc)
    return note_obj

@api_router.put("/notes/{note_id}", response_model=NoteItem)
async def update_note(note_id: str, input_data: NoteCreate):
    update_dict = {
        "title": input_data.title,
        "content": input_data.content,
        "updated_at": datetime.now(timezone.utc).isoformat()
    }
    result = await db.notes.find_one_and_update(
        {"id": note_id},
        {"$set": update_dict},
        return_document=True,
        projection={"_id": 0}
    )
    if not result:
        raise HTTPException(status_code=404, detail="Note not found")
    return result

@api_router.delete("/notes/{note_id}")
async def delete_note(note_id: str):
    result = await db.notes.delete_one({"id": note_id})
    if result.deleted_count == 0:
        raise HTTPException(status_code=404, detail="Note not found")
    return {"status": "deleted"}

# System info
@api_router.get("/system/info")
async def system_info():
    import psutil
    cpu = psutil.cpu_percent(interval=0.1)
    mem = psutil.virtual_memory()
    disk = psutil.disk_usage('/')
    return {
        "cpu_percent": cpu,
        "memory": {
            "total": mem.total,
            "used": mem.used,
            "percent": mem.percent
        },
        "disk": {
            "total": disk.total,
            "used": disk.used,
            "percent": disk.percent
        }
    }

# Seed default file system
@api_router.post("/seed")
async def seed_filesystem():
    count = await db.files.count_documents({})
    if count > 0:
        return {"status": "already seeded"}
    
    defaults = [
        {"name": "Documents", "path": "/Documents", "type": "folder", "content": "", "parent_path": "/"},
        {"name": "Pictures", "path": "/Pictures", "type": "folder", "content": "", "parent_path": "/"},
        {"name": "Music", "path": "/Music", "type": "folder", "content": "", "parent_path": "/"},
        {"name": "Downloads", "path": "/Downloads", "type": "folder", "content": "", "parent_path": "/"},
        {"name": "readme.txt", "path": "/Documents/readme.txt", "type": "file", "content": "Welcome to MineOS!\n\nThis is your personal operating system.\nExplore the desktop, open applications, and enjoy!", "parent_path": "/Documents"},
        {"name": "notes.txt", "path": "/Documents/notes.txt", "type": "file", "content": "My first note in MineOS.", "parent_path": "/Documents"},
    ]
    
    for item in defaults:
        file_obj = FileItem(**item)
        file_obj.size = len(item.get("content", ""))
        await db.files.insert_one(file_obj.model_dump())
    
    return {"status": "seeded"}

app.include_router(api_router)

app.add_middleware(
    CORSMiddleware,
    allow_credentials=True,
    allow_origins=os.environ.get('CORS_ORIGINS', '*').split(','),
    allow_methods=["*"],
    allow_headers=["*"],
)

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

@app.on_event("startup")
async def startup():
    await db.files.create_index("path", unique=True)
    await db.files.create_index("parent_path")
    await db.files.create_index("id", unique=True)
    await db.notes.create_index("id", unique=True)
    count = await db.files.count_documents({})
    if count == 0:
        defaults = [
            {"name": "Documents", "path": "/Documents", "type": "folder", "content": "", "parent_path": "/"},
            {"name": "Pictures", "path": "/Pictures", "type": "folder", "content": "", "parent_path": "/"},
            {"name": "Music", "path": "/Music", "type": "folder", "content": "", "parent_path": "/"},
            {"name": "Downloads", "path": "/Downloads", "type": "folder", "content": "", "parent_path": "/"},
            {"name": "readme.txt", "path": "/Documents/readme.txt", "type": "file", "content": "Welcome to MineOS!\n\nThis is your personal operating system.\nExplore the desktop, open applications, and enjoy!", "parent_path": "/Documents"},
            {"name": "notes.txt", "path": "/Documents/notes.txt", "type": "file", "content": "My first note in MineOS.", "parent_path": "/Documents"},
        ]
        for item in defaults:
            file_obj = FileItem(**item)
            file_obj.size = len(item.get("content", ""))
            await db.files.insert_one(file_obj.model_dump())
        logger.info("File system seeded")

@app.on_event("shutdown")
async def shutdown_db_client():
    client.close()
