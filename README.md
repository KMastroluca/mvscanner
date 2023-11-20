# MVCF Scanning system API design


## Data/Tables
________________
### Resident:   
________________
- __rfid__:     == `String` (17 digit num)
- __name__:     == `String` (Last, First)
- __doc__:      == `String` (982392)
- __room__:     == `String` (e.g 10b) 1 number && 'b' | t''
 __unit:__     == `int`

### `/api/residents`
**GET: Index** `/api/residents`

**GET: SHOW** `/api/residents/{rfid}`

**POST: Create** `/api/residents   body=full payload`

**PATCH: Update** `/api/residents/{rfid}   body={any_updated_fields}`

**DELETE: Delete** `/api/residents/{id}`  



**GET: Index** `/api/residents/{id}/timestamps`
Get all timestamps for X resident DEFAULT= TODAY

**GET: Show** `/api/residents/{id}/timestamps/{start_date}/{end_date}`

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

### `/api/locations/{id}/timestamps/{start_date}/{end_date}`

**GET: Show** Get all timestamps for X location within date range


## Timestamps
### `/api/timestamps`

- __rfid__: `string`
- __dest__: `string`

**GET: Index** `/api/timestamps` Get timestamps for that day (default)

**GET: Show** `/api/timestamps`

**POST: Create** `/api/timestamps/{body=timestamp}`

**GET Show** `/api/timestamps/{start_date}/{end_date}`
