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
        schemasubClassOfs.append({'@id': 'schema:Thing'})



class DataSchema:

    def __init__(self, name, type, comment=''):
        self.key = SearchSchema(name)
        self.subClassOf = {'@id': 'schema:Thing'}
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
        self.comment = comment
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
        if len(self.domainIncludes) == 1:
            self.domainIncludes = self.domainIncludes[0]
        if len(self.rangeIncludes) == 1:
            self.rangeIncludes = self.rangeIncludes[0]
        if self.type == 'rdfs:Class':
            jsonld = {'@id': self.id, '@type': self.type, 'rdfs:comment': self.comment, 'rdfs:label': self.label, 'rdfs:subClassOf': self.subClassOf,'schema:domainIncludes': self.domainIncludes, 'schema:rangeIncludes': self.rangeIncludes}
        elif self.type == 'rdf:Property':
            jsonld = {'@id': self.id, '@type': self.type, 'rdfs:comment': self.comment, 'rdfs:label': self.label, 'schema:domainIncludes': self.domainIncludes, 'schema:rangeIncludes': self.rangeIncludes}
        self.domainIncludes = [self.domainIncludes]
        self.rangeIncludes = [self.rangeIncludes]
        
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

    flag_in_message = 0
    flag_already_added = 0
    flag3 = 0
    jsondata = []
    count = 0

    datalist = f.readlines()
    for data in datalist:
        #count += 1
        #print([count,flag3])
        line = data.split()
        if line: 
            if flag_in_message == 0 and line[0] == 'message':
                flag_in_message = 1
                for i in range(len(jsondata)):
                    if jsondata[i].name == line[1]:
                        tmp_class = jsondata[i]
                        flag3 = 1
                        break
                if flag3 == 0:
                    tmp_class = DataSchema(line[1], 'Class')
                flag3 = 0

            elif flag_in_message == 1 and line[0] != '}':
                for i in range(len(jsondata)):
                    if jsondata[i].name == line[2]:                 # 現在の message Class に含まれる Property が既に存在
                        jsondata[i].addParentClass(tmp_class.name)  # 既にある Property に現在の message Class を親として追加
                        tmp_class.addChildClass(jsondata[i].name)   # 現在の message Class に既にある Property を子として追加
                        flag_already_added = 1
                        break
                if flag_already_added == 0:
                    if len(line) > 6:
                        tmp_prop = DataSchema(line[2], 'Property', ' '.join(line[6:]))
                    else:
                        tmp_prop = DataSchema(line[2], 'Property')
                    tmp_prop.addChildClass(line[1])                 # 新しい Property の中身の Class を追加
                    tmp_prop.addParentClass(tmp_class.name)         # 新しい Property の親として現在の message Class を追加
                    tmp_class.addChildClass(tmp_prop.name)          # 現在の message Class に新しい Property を子として追加
                    for k in range(len(jsondata)):
                        if jsondata[k].name == line[1]:
                            jsondata[k].addParentClass(line[2])
                            flag3 = 1
                            break
                    if flag3 == 0:
                        if line[1] != 'string' and line[1] != 'bytes' and line[1] != 'google.protobuf.Struct' and line[1] != 'google.protobuf.Timestamp':
                            if line[1] == tmp_class.name:
                                tmp_class.addParentClass(line[2])
                            else:
                                tmp3 = DataSchema(line[1], 'Class')
                                tmp3.addParentClass(line[2])
                                jsondata.append(tmp3)
                                del tmp3
                    flag3 = 0
                    jsondata.append(tmp_prop)
                    del tmp_prop
                flag_already_added = 0

            elif flag_in_message == 1 and line[0] == '}':
                flag_in_message = 0
                jsondata.append(tmp_class)
                del tmp_class

    return jsondata



def WriteJsonld(items, jsonldfile):
    with open(jsonldfile, 'w', encoding='UTF-8') as f:
        context = {'dbp': 'http://exdata.co.jp/dbp/schema/', 'rdfs': 'http://www.w3.org/2000/01/rdf-schema#', 'schema': 'https://schema.org/'}
        graph = []
        for item in items:
            if item.key == -1:
                graph.append(item.getJsonld())
        text = {'@context': context, '@graph': graph}

        json.dump(text, f, indent = 2, ensure_ascii=False)




if __name__ == "__main__":
    test = ParseProto('dbp_schema.proto') #入力する.protoファイル名

    WriteJsonld(test, 'dbp-schema.jsonld') #出力する.jsonldファイル名