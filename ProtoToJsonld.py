class DataSchema:

    def __init__(self, name, type):
        self.subClassOf = '"#chincha"'
        self.domainIncludes = []
        self.rangeIncludes = []
        self.type = ''
        self.name = name
        self.id = '"dbp:' + name + '"'
        if type == 'Class':
            self.type = '"rdfs:' + type + '"'
        elif type == 'Property':
            self.type = '"rdf:' + type + '"'
        self.comment = '"#chincha"'
        self.label = '"' + name + '"'

    def addParentClass(self, domainInclude):
        t = '"dbp:' + domainInclude + '"'
        self.domainIncludes.append(t)

    def addChildClass(self, rangeInclude):
        if rangeInclude == 'string':
            t = '"schema:Text"'
        elif rangeInclude == 'google.protobuf.Struct':
            t = '"schema:CreativeWork"'
        elif rangeInclude == 'google.protobuf.Timestamp':
            t = '"schema:DateTime"'
        else:
            t = '"dbp:' + rangeInclude + '"'
        self.rangeIncludes.append(t)

    def getJsonld(self):
        jsonld = '\t\t{\n\t\t\t"@id": ' + self.id + ',\n\t\t\t"@type": ' + self.type + ',\n\t\t\t"@rdfs:comment": ' + self.comment + ',\n\t\t\t"@label": ' + self.label + ',\n'

        if self.type == '"rdfs:Class"':
            jsonld += '\t\t\t"rdfs:subClassOf": {\n\t\t\t\t"@id": ' + self.subClassOf + '\n\t\t\t},\n'
        
        jsonld += '\t\t\t"schema:domainIncludes": [\n'
        for parent in self.domainIncludes:
            jsonld += '\t\t\t\t{\n\t\t\t\t\t"@id": ' + parent + '\n\t\t\t\t},\n'
        jsonld = jsonld.removesuffix(',\n')
        jsonld += '\n\t\t\t],\n'

        jsonld += '\t\t\t"schema:rangeIncludes": [\n'
        for child in self.rangeIncludes:
            jsonld += '\t\t\t\t{\n\t\t\t\t\t"@id": ' + child + '\n\t\t\t\t},\n'
        jsonld = jsonld.removesuffix(',\n')
        jsonld += '\n\t\t\t]\n\t\t},\n'

        return jsonld
 


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
    f = open(jsonldfile, 'w', encoding='UTF-8')

    text = '{\n\t"@context": {\n\t\t"dbp": "http://exdata.co.jp/dbp/schema/",\n\t\t"rdfs": "http://www.w3.org/2000/01/rdf-schema#",\n\t\t"schema": "https://schema.org/"\n\t},\n\t"@graph": [\n'    
    for item in items:
        text += item.getJsonld()
    text = text.removesuffix(',\n')
    text += '\n\t]\n}'

    f.write(text)

    f.close()



if __name__ == "__main__":
    test = ParseProto('dbp_schema.proto') #入力する.protoファイル名

    WriteJsonld(test, 'sampleto1.jsonld') #出力する.jsonldファイル名