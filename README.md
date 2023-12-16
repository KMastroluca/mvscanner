# MVCF Scanning system API design

## Data/Tables


### Resident:

---

- **rfid**: == `String` (17 digit num)
- **name**: == `String` (Last, First)
- **doc**: == `String` (982392)
- **room**: == `String` (e.g 10b) 1 number && 'b' | t''
- **unit:** == `int`
- **current_location** == `int`

### `/api/residents`

**GET: Index** `/api/residents`

**GET: SHOW** `/api/residents/{rfid}`

**POST: Create** `/api/residents   body=JSON:Resident`

**PATCH: Update** `/api/residents/{rfid}   body=JSON: any_updated_fields`

**DELETE: Delete** `/api/residents/{id}`

**GET: Index** `/api/residents/{rfid}/timestamps`
Get all timestamps for X resident DEFAULT= TODAY

**Index Query Params** `?unique=true` // get only Last timestamp for each resident

**GET: Show** `/api/residents/{rfid}/timestamps/{start_date}/{end_date}`

========================================================

## Locations:

#### `/api/locations`

- `id`: _int_ e.g. (6)
- `name`: _string_ e.g. (DeltaPod)
- `level`: __int__ e.g. (3)

**GET: Index** (all locations)

**GET: Show** `/api/locations{id}` Get the name of location X

**POST: Create** `/api/locations{body=JSON location}` Add a new location to sign out to

### `/api/locations/{id}/timestamps`

**GET: Show** Get all timestamps for X location DEFAULT= TODAY

**Query Params** `?unique=true` // get only Last timestamp for each resident

### `/api/locations/{id}/timestamps/{start_date}/{end_date}`

**GET: Show** Get all timestamps for X location within date range

### Show all residents that live at location X
`GET: /api/locations/{id}/residents?current=true`

## Timestamps

### `/api/timestamps`

- **rfid**: `string`
- **dest**: `string`

**GET: Index** `/api/timestamps`?unique=true
Get timestamps for that day (default)

**GET: Show** `/api/timestamps`

**POST: Create** `/api/timestamps/{body=timestamp}`

TIMESTAMP (Sent by front-end)

```json
[
  {
    "rfid": "12345678901234567",
    "location": 8
  }
]
```

RETURNS: (from back-end)

```json
{
  "rfid": "12345678901234567",
  "name": "Doe, John",
  "doc": "247823",
  "room": "B13t",
  "unit": 2,
  "current_location": 8
}
```

#### IF two timestamps are received at the same location, the location returned will be 0: "AWAY" and that can be checked for on the front-end, and you can send another timestamp after prompting the user where they are going, and then that timestamp will be returned with their updated location. This only if a user is going to a location that does not have a scanner.

**GET Show** `/api/timestamps/{start_date}/{end_date}`
