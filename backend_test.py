import requests
import sys
import json
from datetime import datetime

class MineOSAPITester:
    def __init__(self, base_url="https://graphical-kernel.preview.emergentagent.com/api"):
        self.base_url = base_url
        self.tests_run = 0
        self.tests_passed = 0
        self.test_results = []

    def run_test(self, name, method, endpoint, expected_status, data=None):
        """Run a single API test"""
        url = f"{self.base_url}/{endpoint}" if endpoint else self.base_url
        headers = {'Content-Type': 'application/json'}

        self.tests_run += 1
        print(f"\n🔍 Testing {name}...")
        
        try:
            if method == 'GET':
                response = requests.get(url, headers=headers, timeout=10)
            elif method == 'POST':
                response = requests.post(url, json=data, headers=headers, timeout=10)
            elif method == 'PUT':
                response = requests.put(url, json=data, headers=headers, timeout=10)
            elif method == 'DELETE':
                response = requests.delete(url, headers=headers, timeout=10)

            success = response.status_code == expected_status
            if success:
                self.tests_passed += 1
                print(f"✅ Passed - Status: {response.status_code}")
                try:
                    response_data = response.json()
                    print(f"   Response: {json.dumps(response_data, indent=2)[:200]}...")
                except:
                    print(f"   Response: {response.text[:200]}...")
            else:
                print(f"❌ Failed - Expected {expected_status}, got {response.status_code}")
                print(f"   Response: {response.text[:200]}...")

            self.test_results.append({
                "test": name,
                "method": method,
                "endpoint": endpoint,
                "expected_status": expected_status,
                "actual_status": response.status_code,
                "success": success,
                "response_preview": response.text[:100] if response.text else ""
            })

            return success, response.json() if success and response.text else {}

        except Exception as e:
            print(f"❌ Failed - Error: {str(e)}")
            self.test_results.append({
                "test": name,
                "method": method,
                "endpoint": endpoint,
                "expected_status": expected_status,
                "actual_status": "ERROR",
                "success": False,
                "error": str(e)
            })
            return False, {}

    def test_root_api(self):
        """Test root API endpoint"""
        return self.run_test("Root API", "GET", "", 200)

    def test_system_info(self):
        """Test system info endpoint"""
        return self.run_test("System Info", "GET", "system/info", 200)

    def test_get_files(self):
        """Test get files endpoint"""
        return self.run_test("Get Files", "GET", "files", 200)

    def test_get_all_files(self):
        """Test get all files endpoint"""
        return self.run_test("Get All Files", "GET", "files/all", 200)

    def test_seed_filesystem(self):
        """Test seed filesystem endpoint"""
        return self.run_test("Seed Filesystem", "POST", "seed", 200)

    def test_create_file(self):
        """Test create file endpoint"""
        test_file = {
            "name": "test_file.txt",
            "path": "/test_file.txt",
            "type": "file",
            "content": "This is a test file created by the testing agent.",
            "parent_path": "/"
        }
        success, response = self.run_test("Create File", "POST", "files", 201, test_file)
        return success, response.get('id') if success else None

    def test_update_file(self, file_id):
        """Test update file endpoint"""
        if not file_id:
            print("❌ Skipping update test - no file ID")
            return False, {}
        
        update_data = {
            "content": "Updated content for test file.",
            "name": "updated_test_file.txt"
        }
        return self.run_test("Update File", "PUT", f"files/{file_id}", 200, update_data)

    def test_delete_file(self, file_id):
        """Test delete file endpoint"""
        if not file_id:
            print("❌ Skipping delete test - no file ID")
            return False, {}
        
        return self.run_test("Delete File", "DELETE", f"files/{file_id}", 200)

    def test_notes_crud(self):
        """Test notes CRUD operations"""
        # Get notes
        success, _ = self.run_test("Get Notes", "GET", "notes", 200)
        
        # Create note
        test_note = {
            "title": "Test Note",
            "content": "This is a test note created by the testing agent."
        }
        success, response = self.run_test("Create Note", "POST", "notes", 200, test_note)
        note_id = response.get('id') if success else None
        
        if note_id:
            # Update note
            update_note = {
                "title": "Updated Test Note",
                "content": "This note has been updated."
            }
            self.run_test("Update Note", "PUT", f"notes/{note_id}", 200, update_note)
            
            # Delete note
            self.run_test("Delete Note", "DELETE", f"notes/{note_id}", 200)

def main():
    print("🚀 Starting MineOS API Testing...")
    tester = MineOSAPITester()

    # Test basic endpoints
    tester.test_root_api()
    tester.test_system_info()
    tester.test_seed_filesystem()
    
    # Test file operations
    tester.test_get_files()
    tester.test_get_all_files()
    
    # Test file CRUD
    success, file_id = tester.test_create_file()
    tester.test_update_file(file_id)
    tester.test_delete_file(file_id)
    
    # Test notes CRUD
    tester.test_notes_crud()

    # Print final results
    print(f"\n📊 Final Results: {tester.tests_passed}/{tester.tests_run} tests passed")
    
    # Save detailed results
    with open('/app/test_reports/backend_api_results.json', 'w') as f:
        json.dump({
            "timestamp": datetime.now().isoformat(),
            "total_tests": tester.tests_run,
            "passed_tests": tester.tests_passed,
            "success_rate": f"{(tester.tests_passed/tester.tests_run)*100:.1f}%",
            "test_details": tester.test_results
        }, f, indent=2)
    
    return 0 if tester.tests_passed == tester.tests_run else 1

if __name__ == "__main__":
    sys.exit(main())