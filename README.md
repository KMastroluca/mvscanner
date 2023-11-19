# MVCF Scanning system API design


## Data/Tables
________________
### Resident:   
________________
- __RFID__:     == `String` (17 digit num)
- __Name__:     == `String` (Last, First)
- __DOC__:      == `String` (982392)
- __Pod__:      == `String` ("A" | "B" | "C")
- __Room__:     == `String` (e.g 10b) 1 number && 'b' | t''

### `/api/residents`
**GET: Index** `/api/residents`

**POST: Create** `/api/residents/{body=full payload}`

**PUT: Update** `/api/residents/{id}/{body=updates}`

**DELETE: Delete** `/api/residents/{id}`  

### `/api/residents/{id}/timestamps`

**GET: Index** Get all timestamps for resident{id}

(LOL silly rabbit. request bodies aren't for get methods)

========================================================
## Locations:  
#### `/api/locations`

- `id`: _int_   e.g. (6)
- `name`: _string_ e.g. (DeltaPod)

**GET: Index** (all locations)

**GET: Show** `/api/locations{id}` Get the name of location X

**POST: Create** `/api/locations{body=full_payload}` Add a new location to sign out to

### `/api/locations/{id}/timestamps`

**GET: Show** Get all timestamps for X location DEFAULT= TODAY

## Timestamps
### `/api/timestamps`

- __rfid__: `string`
- __destination_loc__: `String`
- __Timestamp__: `String` (pretty printed but the DB will also store a timestamp we wont see)

**GET: Index** `/api/timestamps` Get timestamps for that day (default)

**GET: Show** `/api/timestamps`

#### __RANGE:__
```JSON
{
  "date": {
    "start": "MM/DD/YYYY",
    "end": "MM/DD/YYYY",
  }
}
```
**POST: Create** `/api/timestamps/{body=timestamp}`

