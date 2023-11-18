import unittest
import requests
import json


class TestResidentsController(unittest.TestCase):
    base_url = "http://localhost:8080/api/residents"

    def test_get_all_residents(self):
        response = requests.get(self.base_url)
        self.assertEqual(response.status_code, 200)
        self.assertIsInstance(response.json(), list)

    def test_get_single_resident(self):
        resident_id = "123455623562354"
        response = requests.get(f"{self.base_url}/{resident_id}")
        self.assertEqual(response.status_code, 200)
        self.assertEqual(response.json()["rfid"], resident_id)

    def test_post_new_resident(self):
        fake_resident = {
            "rfid": "999999999999",
            "name": "Fake Resident",
            "doc": "999999",
            "room": "C-3",
        }
        response = requests.post(self.base_url, json=fake_resident)
        self.assertEqual(response.status_code, 201)  # Assuming 201 for created

    def test_put_update_resident(self):
        resident_id = "123455623562354"
        updated_data = {"name": "Updated Name"}
        response = requests.put(f"{self.base_url}/{resident_id}", json=updated_data)
        self.assertEqual(response.status_code, 200)
        self.assertEqual(response.json()["name"], "Updated Name")

    def test_delete_resident(self):
        resident_id = "123455623562354"
        response = requests.delete(f"{self.base_url}/{resident_id}")
        self.assertEqual(response.status_code, 204)  # Assuming 204 for no content


class TestLocationsController(unittest.TestCase):
    base_url = "http://localhost:8080/api/locations"

    def test_get_all_locations(self):
        response = requests.get(self.base_url)
        self.assertEqual(response.status_code, 200)
        self.assertIsInstance(response.json(), list)

    def test_post_new_location(self):
        fake_location = {"name": "New Location"}
        response = requests.post(self.base_url, json=fake_location)
        self.assertEqual(response.status_code, 201)  # Assuming 201 for created

    def test_put_update_location(self):
        location_id = 12
        updated_data = {"name": "Updated Location Name"}
        response = requests.put(f"{self.base_url}/{location_id}", json=updated_data)
        self.assertEqual(response.status_code, 200)
        self.assertEqual(response.json()["name"], "Updated Location Name")

    def test_delete_location(self):
        location_id = 12
        response = requests.delete(f"{self.base_url}/{location_id}")
        self.assertEqual(response.status_code, 204)  # Assuming 204 for no content


class TestTimestampsController(unittest.TestCase):
    base_url = "http://localhost:8080/api/timestamps"

    def test_crud_timestamp(self):
        fake_timestamp = {
            "rfid": "666666000666666",
            "dest": 19,
            "time": "2021-02-02T12:12:12",
        }

        # Create
        response = requests.post(self.base_url, json=fake_timestamp)
        self.assertEqual(response.status_code, 201)  # Assuming 201 for created
        timestamp_id = response.json()["id"]

        # Read
        response = requests.get(f"{self.base_url}/{timestamp_id}")
        self.assertEqual(response.status_code, 200)
        self.assertEqual(response.json()["rfid"], "666666000666666")

        # Update
        updated_data = {"dest": 20}
        response = requests.put(f"{self.base_url}/{timestamp_id}", json=updated_data)
        self.assertEqual(response.status_code, 200)
        self.assertEqual(response.json()["dest"], 20)

        # Delete
        response = requests.delete(f"{self.base_url}/{timestamp_id}")
        self.assertEqual(response.status_code, 204)  # Assuming 204 for no content


if __name__ == "__main__":
    unittest.main()
