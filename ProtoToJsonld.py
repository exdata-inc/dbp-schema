import json
import requests

URL = 'https://github.com/schemaorg/schemaorg/blob/main/data/releases/21.0/schemaorg-current-https.jsonld?raw=true'
response = requests.get(URL)
origin = json.loads(response.text)
#with open('sampleto3.jsonld', 'w', encoding='UTF-8') as fj:
#        json.dump(origin, fj, indent = 2)

schemalist = origin['@graph']
schemaids = []
schemacomments = []
schemasubClassOfs = []

for elm in schemalist:
    schemaids.append(elm['@id'])
    schemacomments.append(elm['rdfs:comment'])
    if 'rdfs:subClassOf' in elm.keys():
        schemasubClassOfs.append(elm['rdfs:subClassOf'])
    else:
        schemasubClassOfs.append({'@id': '#chincha'})



class DataSchema:

    def __init__(self, name, type):
        self.key = SearchSchema(name)
        self.subClassOf = {'@id': '#chincha'}
        if self.key != -1:
            self.subClassOf = schemasubClassOfs[self.key]
        self.domainIncludes = []
        self.rangeIncludes = []
        self.type = ''
        self.name = name
        self.id = 'dbp:' + name
        if self.key != -1:
            self.id = schemaids[self.key]
        if type == 'Class':
            self.type = 'rdfs:' + type
        elif type == 'Property':
            self.type = 'rdf:' + type
        self.comment = '#chincha'
        if self.key != -1:
            self.comment = schemacomments[self.key]
        self.label = name

    def addParentClass(self, domainInclude):
        t = {'@id': DbpOrSchema(domainInclude)}
        self.domainIncludes.append(t)

    def addChildClass(self, rangeInclude):
        if rangeInclude == 'string':
            t = {'@id': 'schema:Text'}
        elif rangeInclude == 'google.protobuf.Struct':
            t = {'@id': 'schema:CreativeWork'}
        elif rangeInclude == 'google.protobuf.Timestamp':
            t = {'@id': 'schema:DateTime'}
        else:
            t = {'@id': DbpOrSchema(rangeInclude)}
        self.rangeIncludes.append(t)

    def getJsonld(self):
        if self.type == 'rdfs:Class':
            jsonld = {'@id': self.id, '@type': self.type, 'rdfs:comment': self.comment, 'rdfs:label': self.label, 'rdfs:subClassOf': self.subClassOf,'schema:domainIncludes': self.domainIncludes, 'schema:rangeIncludes': self.rangeIncludes}
        elif self.type == 'rdf:Property':
            jsonld = {'@id': self.id, '@type': self.type, 'rdfs:comment': self.comment, 'rdfs:label': self.label, 'schema:domainIncludes': self.domainIncludes, 'schema:rangeIncludes': self.rangeIncludes}

        return jsonld



def SearchSchema(name):
    for i in range(len(schemaids)):
        if schemaids[i] == 'schema:' + name:
            return i
    return -1



def DbpOrSchema(name):
    for filter in schemaids:
        if filter == 'schema:' + name:
            return filter
    return 'dbp:' + name



def ParseProto(protofile):
    f = open(protofile, 'r', encoding='UTF-8')

    flag1 = 0
    flag2 = 0
    flag3 = 0
    jsondata = []
    count = 0

    datalist = f.readlines()
    for data in datalist:
        #count += 1
        #print([count,flag3])
        line = data.split()
        if line: 
            if flag1 == 0 and line[0] == 'message':
                flag1 = 1
                for i in range(len(jsondata)):
                    if jsondata[i].name == line[1]:
                        tmp1 = jsondata[i]
                        flag3 = 1
                        break
                if flag3 == 0:
                    tmp1 = DataSchema(line[1], 'Class')
                flag3 = 0

            elif flag1 == 1 and line[0] != '}':
                for i in range(len(jsondata)):
                    if jsondata[i].name == line[2]:
                        jsondata[i].addParentClass(tmp1.name)
                        tmp1.addChildClass(jsondata[i].name)
                        flag2 = 1
                        break
                if flag2 == 0:
                    tmp2 = DataSchema(line[2], 'Property')
                    tmp2.addChildClass(line[1])
                    tmp2.addParentClass(tmp1.name)
                    tmp1.addChildClass(tmp2.name)
                    for k in range(len(jsondata)):
                        if jsondata[k].name == line[1]:
                            jsondata[k].addParentClass(line[2])
                            flag3 = 1
                            break
                    if flag3 == 0:
                        if line[1] != 'string' and line[1] != 'bytes' and line[1] != 'google.protobuf.Struct' and line[1] != 'google.protobuf.Timestamp':
                            if line[1] == tmp1.name:
                                tmp1.addParentClass(line[2])
                            else:
                                tmp3 = DataSchema(line[1], 'Class')
                                tmp3.addParentClass(line[2])
                                jsondata.append(tmp3)
                                del tmp3
                    flag3 = 0
                    jsondata.append(tmp2)
                    del tmp2
                flag2 = 0

            elif flag1 == 1 and line[0] == '}':
                flag1 = 0
                jsondata.append(tmp1)
                del tmp1

    return jsondata



def WriteJsonld(items, jsonldfile):
    with open(jsonldfile, 'w', encoding='UTF-8') as f:
        context = {'dbp': 'http://exdata.co.jp/dbp/schema/', 'rdfs': 'http://www.w3.org/2000/01/rdf-schema#', 'schema': 'https://schema.org/'}
        graph = []
        for item in items:
            graph.append(item.getJsonld())
        text = {'@context': context, '@graph': graph}

        json.dump(text, f, indent = 2)




if __name__ == "__main__":
    test = ParseProto('dbp_schema.proto') #入力する.protoファイル名

    WriteJsonld(test, 'sampleto2.jsonld') #出力する.jsonldファイル名