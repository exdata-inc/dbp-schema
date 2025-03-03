# dbp-schema

JSON-LD schema of metadata for [DataBrewingPlatform](https://github.com/exdata-inc/DataBrewingPlatform)

It also contains schema for RWD Profile (`dbp:RealWorldDataFieldProfile`), which describes structure of Real-World Data.

## Build

### for Rust

```
cargo build
```

### for Go-lang

```
protoc -I . --go_out=paths=source_relative:. --experimental_allow_proto3_optional  dbp_schema.proto
```

## RWD Profile Example

### CSV

- Data

  ```.csv
  2023-01-01T00:00:00,6909fcbb-ef24-4f9b-8981-4a81297cb133,100,35.987903,136.88374
  2023-01-01T00:01:02,ba3364b5-662e-4909-b6d1-0e59d8b34beb,101,35.987008,136.88371
  2023-01-01T00:01:58,72b81471-ed17-4f2d-8df7-1249f7daf08f,102,35.988905,136.83374
  2023-01-01T00:03:00,2159cb54-b22d-4fa8-8a12-7eeeb41f4348,103,35.967909,136.88344
  2023-01-01T00:04:01,98f1b873-31fc-4d6c-a3c6-3ca562830c14,104,35.917918,136.85375
  ...
  ```

- RWD Profile
  ```.json
  {
    "@id": "https://path.to/this.jsonld",
    "@type": "dbp:RealWorldDataFieldProfile",
    "@context": {
      "dbp": "https://exdata.co.jp/dbp/schema/",
      "rdf": "https://www.w3.org/1999/02/22-rdf-syntax-ns#",
      "rdfs": "http://www.w3.org/2000/01/rdf-schema#",
      "schema": "https://schema.org/"
    },
    "schema:name": "Sample RWD Profile for CSV",
    "dbp:structure": {
      "@type": "dbp:RealWorldDataStructureGraph",
      "@graph": [
        {
          "@id": "root",
          "@type": "rdf:List",
          "rdfs:label": "root",
          "schema:rangeIncludes": {
            "@id": "class1"
          },
          "dbp:dbpaCompress": true,
          "dbp:dbpaCompressListID": 0
        },
        {
          "@id": "class1",
          "@type": "rdfs:Class",
          "rdfs:label": "class1",
          "schema:rangeIncludes": [
            {
              "@id": "datetime"
            },
            {
              "@id": "id"
            },
            {
              "@id": "seq"
            },
            {
              "@id": "latitude"
            },
            {
              "@id": "longitude"
            }
          ],
          "schema:domainIncludes": {
            "@id": "root"
          },
          "dbp:dbpaCompressRowID": 0
        },
        {
          "@id": "datetime",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "datetime",
          "dbp:itemType": "DateTime",
          "rdfs:comment": "Timestamp when an item was recorded.",
          "schema:rangeIncludes": {
            "@id": "schema:Date"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:VariableCharacteristicEnumeration": "Interval",
          "dbp:dbpaDateTimeFormat": "%Y-%m-%dT%H:%M:%S",
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 0
        },
        {
          "@id": "id",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "id",
          "dbp:itemType": "String(32)",
          "rdfs:comment": "Unique identifier for each item.",
          "schema:rangeIncludes": {
            "@id": "schema:Text"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:VariableCharacteristicEnumeration": "Nominal",
          "dbp:isEnumValue": false,
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 1
        },
        {
          "@id": "seq",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "seq",
          "dbp:itemType": "Integer",
          "rdfs:comment": "Sequential Number of data record.",
          "schema:rangeIncludes": {
            "@id": "schema:Integer"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:VariableCharacteristicEnumeration": "Ordinal",
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 2,
          "dbp:RangeMin": "100",
          "dbp:RangeMax": "999999",
          "dbp:BaseIncrement": 1,
          "dbp:UseBaseIncrement": true
        },
        {
          "@id": "latitude",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "latitude",
          "dbp:itemType": "Float",
          "rdfs:comment": "Geographic coordinate representing north-south position",
          "schema:unitText": "DD",
          "schema:rangeIncludes": {
            "@id": "schema:Float"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:VariableCharacteristicEnumeration": "Proportional",
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 3,
          "dbp:RangeMin": "35.15300000",
          "dbp:RangeMax": "35.15999999",
          "dbp:DecimalPlaces:": 6,
          "dbp:PrecisionBytes": 2
        },
        {
          "@id": "longitude",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "longitude",
          "dbp:itemType": "Float",
          "rdfs:comment": "Geographic coordinate representing east-west position",
          "schema:unitText": "DD",
          "schema:rangeIncludes": {
            "@id": "schema:Float"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:VariableCharacteristicEnumeration": "Interval",
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 4,
          "dbp:RangeMin": "136.9700000",
          "dbp:RangeMax": "136.9899999",
          "dbp:DecimalPlaces:": 5,
          "dbp:PrecisionBytes": 2
        }
      ]
    },
    "schema:dateCreated": "2024-12-31T23:59:59.999999+09:00",
    "schema:encodingFormat": "text/csv",
    "dbp:csvHasHeader": false,
    "dbp:newLineCharacter": "LF"
  }
  ```

### JSON

- Data
  ```.json
  [
    {
      "dateStart": "2023-01-01T00:00:00",
      "dateEnd": "2023-01-01T01:59:59",
      "id": "6909fcbb-ef24-4f9b-8981-4a81297cb133",
      "deviceType": "ABC",
      "positions": [
        {
          "latitude": 35.9879087,
          "longitude": 136.883749
        },
        {
          "latitude": 35.9879086,
          "longitude": 136.883751
        },
        ...
      ]
    },
    {
      "dateStart": "2023-01-01T00:01:02",
      "dateEnd": "2023-01-01T02:01:01",
      "id": "ba3364b5-662e-4909-b6d1-0e59d8b34beb",
      "deviceType": "DEF",
      "positions": [
        {
          "latitude": 35.9879035,
          "longitude": 136.883732
        },
        {
          "latitude": 35.9879037,
          "longitude": 136.883730
        },
        ...
      ]
    },
    ...
  ]
  ```

- RWD Profile
  ```.json
  {
    "@id": "https://path.to/this.jsonld",
    "@type": "dbp:RealWorldDataFieldProfile",
    "@context": {
      "dbp": "https://exdata.co.jp/dbp/schema/",
      "rdf": "https://www.w3.org/1999/02/22-rdf-syntax-ns#",
      "rdfs": "http://www.w3.org/2000/01/rdf-schema#",
      "schema": "https://schema.org/"
    },
    "schema:name": "Sample RWD Profile for JSON",
    "dbp:structure": {
      "@type": "dbp:RealWorldDataStructureGraph",
      "@graph": [
        {
          "@id": "root",
          "@type": "rdf:List",
          "rdfs:label": "root",
          "schema:rangeIncludes": {
            "@id": "class1"
          },
          "dbp:dbpaCompress": true,
          "dbp:dbpaCompressListID": 0
        },
        {
          "@id": "class1",
          "@type": "rdfs:Class",
          "rdfs:label": "class1",
          "schema:rangeIncludes": [
            {
              "@id": "dateStart"
            },
            {
              "@id": "dateEnd"
            },
            {
              "@id": "id"
            },
            {
              "@id": "deviceType"
            },
            {
              "@id": "points"
            }
          ],
          "schema:domainIncludes": {
            "@id": "root"
          },
          "dbp:dbpaCompressRowID": 0
        },
        {
          "@id": "dateStart",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "dateStart",
          "dbp:itemType": "DateTime",
          "rdfs:comment": "Timestamp when an item was started to record.",
          "schema:rangeIncludes": {
            "@id": "schema:Date"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:VariableCharacteristicEnumeration": "Interval",
          "dbp:dbpaDateTimeFormat": "%Y-%m-%dT%H:%M:%S",
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 0
        },
        {
          "@id": "dateEnd",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "dateEnd",
          "dbp:itemType": "DateTime",
          "rdfs:comment": "Timestamp when an item was ended to record.",
          "schema:rangeIncludes": {
            "@id": "schema:Date"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:VariableCharacteristicEnumeration": "Interval",
          "dbp:dbpaDateTimeFormat": "%Y-%m-%dT%H:%M:%S",
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 1
        },
        {
          "@id": "id",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "id",
          "dbp:itemType": "String(32)",
          "rdfs:comment": "Unique identifier for each item.",
          "schema:rangeIncludes": {
            "@id": "schema:Text"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:VariableCharacteristicEnumeration": "Nominal",
          "dbp:isEnumValue": false,
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 2
        },
        {
          "@id": "deviceType",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "deviceType",
          "dbp:itemType": "String(3)",
          "rdfs:comment": "Type of the device.",
          "schema:rangeIncludes": {
            "@id": "schema:Text"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:VariableCharacteristicEnumeration": "Nominal",
          "dbp:isEnumValue": true,
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 3
        },
        {
          "@id": "points",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "points",
          "dbp:itemType": "list",
          "rdfs:comment": "Location history of a record.",
          "schema:rangeIncludes": {
            "@id": "list2"
          },
          "schema:domainIncludes": {
            "@id": "class1"
          },
          "dbp:dbpaCompressParentListID": 0,
          "dbp:dbpaCompressColumnID": 4,
          "dbp:dbpaChildrenLists": [
            1
          ]
        },
        {
          "@id": "list2",
          "@type": "rdf:List",
          "rdfs:label": "list2",
          "schema:domainIncludes": {
            "@id": "points"
          },
          "schema:rangeIncludes": {
            "@id": "class2"
          },
          "dbp:dbpaCompress": true,
          "dbp:dbpaCompressListID": 1
        },
        {
          "@id": "class2",
          "@type": "rdfs:Class",
          "rdfs:label": "class2",
          "schema:rangeIncludes": [
            {
              "@id": "latitude"
            },
            {
              "@id": "longitude"
            }
          ],
          "schema:domainIncludes": {
            "@id": "list2"
          },
          "dbp:dbpaCompressRowID": 1
        },
        {
          "@id": "latitude",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "latitude",
          "dbp:itemType": "Float",
          "rdfs:comment": "Geographic coordinate representing north-south position",
          "schema:unitText": "DD",
          "schema:rangeIncludes": {
            "@id": "schema:Float"
          },
          "schema:domainIncludes": {
            "@id": "class2"
          },
          "dbp:VariableCharacteristicEnumeration": "Proportional",
          "dbp:dbpaCompressParentListID": 1,
          "dbp:dbpaCompressColumnID": 0,
          "dbp:RangeMin": "35.153000000",
          "dbp:RangeMax": "35.159999999",
          "dbp:DecimalPlaces:": 6,
          "dbp:PrecisionBytes": 2
        },
        {
          "@id": "longitude",
          "@type": "dbp:RealWorldDataStructureProperty",
          "rdfs:label": "longitude",
          "dbp:itemType": "Float",
          "rdfs:comment": "Geographic coordinate representing east-west position",
          "schema:unitText": "DD",
          "schema:rangeIncludes": {
            "@id": "schema:Float"
          },
          "schema:domainIncludes": {
            "@id": "class2"
          },
          "dbp:VariableCharacteristicEnumeration": "Interval",
          "dbp:dbpaCompressParentListID": 1,
          "dbp:dbpaCompressColumnID": 1,
          "dbp:RangeMin": "136.97000000",
          "dbp:RangeMax": "136.98999999",
          "dbp:DecimalPlaces:": 7,
          "dbp:PrecisionBytes": 2
        }
      ]
    },
    "schema:dateCreated": "2024-12-31T23:59:59.999999+09:00",
    "schema:encodingFormat": "application/json"
  }
  ```
