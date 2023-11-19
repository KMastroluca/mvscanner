import json
import unittest
import requests


class TestResidentsController(unittest.TestCase):
    """
    .service(residents_controller::index)
    .service(residents_controller::show)
    .service(residents_controller::show_resident_timestamps)
    .service(residents_controller::store)
    .service(residents_controller::destroy)
    .service(residents_controller::update)
    """

    base_url = "http://localhost:8080/api/residents"

    def test_index(self):
        response = requests.get(self.base_url)
        self.assertEqual(response.status_code, 200)
        self.assertIsInstance(response.json(), list)

    def test_show(self):
        resident_id = "888888222888888"
        response = requests.get(f"{self.base_url}/{resident_id}")
        self.assertEqual(response.status_code, 200)
        self.assertEqual(response.json()["rfid"], resident_id)

    def test_store(self):
        fake_resident = {
            "rfid": "888888222888844",
            "name": "Fake Resident",
            "doc": "999999",
            "room": "C-3",
            "unit": 8,
        }
        response = requests.post(self.base_url, json=fake_resident)
        self.assertEqual(response.status_code, 201)  # Assuming 201 for created

    def test_update(self):
        resident_id = "888888222888800"
        updated_data = {"name": "Updated Name"}
        response = requests.patch(f"{self.base_url}/{resident_id}", json=updated_data)
        self.assertEqual(response.status_code, 200)

    def test_delete_resident(self):
        resident_id = "333333333333333"
        response = requests.delete(f"{self.base_url}/{resident_id}")
        self.assertEqual(response.status_code, 204)  # Assuming 204 for no content


class TestLocationsController(unittest.TestCase):

    """locations_controller::index
    locations_controller::show
    locations_controller::store
    locations_controller::show_location_timestamps
    locations_controller::show_location_timestamps_range
    locations_controller::show_location_residents
    """

    base_url = "http://localhost:8080/api/locations"

    def test_index(self):
        response = requests.get(self.base_url)
        self.assertEqual(response.status_code, 200)
        self.assertIsInstance(response.json(), list)

    def test_show(self):
        response = requests.get(f"{self.base_url}/4")
        self.assertEqual(response.status_code, 200)
        data = response.json().get("name")
        self.assertEqual(data, "ASU")

    def test_store(self):
        fake_location = {"id": 69, "name": "ur_moms_house"}
        response = requests.post(self.base_url, json=fake_location)
        self.assertEqual(response.status_code, 201)

    def test_show_location_timestamps(self):
        response = requests.get(f"{self.base_url}/8/timestamps")
        self.assertEqual(response.status_code, 200)
        self.assertIsInstance(response.json(), list)

    def test_show_location_timestamps_range(self):
        response = requests.get(f"{self.base_url}/13/timestamps/2023-11-19/2023-11-20")
        self.assertEqual(response.status_code, 200)
        self.assertIsInstance(response.json(), list)

    def test_show_location_residents(self):
        response = requests.get(f"{self.base_url}/8/residents")
        self.assertEqual(response.status_code, 200)


class TestTimestampsController(unittest.TestCase):
    base_url = "http://localhost:8080/api/timestamps"

    def test_index(self):
        response = requests.get(self.base_url)
        self.assertEqual(response.status_code, 200)
        self.assertIsInstance(response.json(), list)

    def test_show(self):
        response = requests.get(f"{self.base_url}/1")
        self.assertEqual(response.status_code, 200)
        self.assertEqual(response.json()["rfid"], "888888222888888")

    def test_store(self):
        fake_timestamp = {"rfid": "666666000666666", "dest": 19}
        response = requests.post(self.base_url, json=fake_timestamp)
        self.assertEqual(response.status_code, 201)  # Assuming 201 for created

    def test_show_timestamps_range(self):
        response = requests.get(f"{self.base_url}/2023-11-19/2023-11-20")
        self.assertEqual(response.status_code, 200)
        self.assertIsInstance(response.json(), list)


if __name__ == "__main__":
    unittest.main()
