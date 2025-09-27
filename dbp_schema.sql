Table RWDataset [headercolor: #FF7700] {
    id URL [pk]                                 // 自身(このJSON-LD)のURL
    name VARCHAR(256)                           // (人間が見てわかりやすい)データセットの名前
    url URL                                     // (未使用)
    structureInfo RWDataFieldProfile    // 構造情報
    generatedFrom RWDataset              // このデータセットの基になったデータセット(醸造時の入力データセット)
    generatedUsing RWDataBrewerInfo      // このデータセットを作った醸造プログラム
    generatedArgs RWDataBrewingArgument  // このデータセットを作った際のパラメータ(醸造時の入力パラメータ)
    collectionInfo RWDataCollectionInfo  // 収集情報
    distribution RWDataStoringInfo       // データの保存先（データの記録期間に応じて、複数の場所に置く場合もあるため repeated）
    author VARCHAR(256)                         // データを集めている主体
    contentLocation VARCHAR(256)                // データを集めている場所
    dateCreated Timestamp                       // データを集め始めた日時
    dateModified Timestamp                      // 最新のデータを集め始めた日時
    datePublished Timestamp 
    license VARCHAR(256) 
    locationCreated VARCHAR(256) 
    description VARCHAR(256) 
}

Ref: RWDataset.structureInfo  > RWDataFieldProfile.id
Ref: RWDataset.generatedFrom  > RWDataset.id
Ref: RWDataset.generatedUsing > RWDataBrewerInfo.id
Ref: RWDataset.generatedArgs  > RWDataBrewingArgument.id
Ref: RWDataset.collectionInfo > RWDataCollectionInfo.id
Ref: RWDataset.distribution   > RWDataStoringInfo.id


Table RWDataBrewerInput [headercolor: #C00000] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    key VARCHAR(256) 
    inputType VARCHAR(256) 
    value bytes 
    inputCharacteristic VariableCharacteristicEnumeration 
    dataset RWDataset 
}

Ref: RWDataBrewerInput.dataset  > RWDataset.id


Table RWDataBrewerOutput [headercolor: #C00000] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    key VARCHAR(256) 
    outputType VARCHAR(256) 
    value bytes 
    outputCharacteristic VariableCharacteristicEnumeration 
}



Table RWDataBrewingArgument [headercolor: #C00000] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    key VARCHAR(256) 
    argumentType VARCHAR(256) 
    value bytes 
}


Table RWDataBrewerInfo [headercolor: #C00000] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    inputSpecs RWDataBrewerInput 
    outputSpecs RWDataBrewerOutput 
    argSpecs RWDataBrewingArgument 
    conversionCharacteristic ConversionCharacteristicEnumeration 
}

Ref: RWDataBrewerInfo.inputSpecs  > RWDataBrewerInput.id
Ref: RWDataBrewerInfo.outputSpecs > RWDataBrewerOutput.id
Ref: RWDataBrewerInfo.argSpecs    > RWDataBrewingArgument.id



Table RWDataCollectionInfo [headercolor: #00B0F0] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    collectionStyle VARCHAR(256) 
    collectionProtocol VARCHAR(256) 
    listenAddress VARCHAR(256) 
    serverAddress VARCHAR(256) 
    entryPoint EntryPoint 
}


Table RWDataFieldProfile [headercolor: #00B0F0] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    encodingFormat VARCHAR(256) 
    structure JSON  // JSON-LD schema ( including @graph )
    graphqlSchema VARCHAR(256) 
}

Table RWDataStoringInfo [headercolor: #2F5597] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    startTime Timestamp 
    endTime Timestamp 
    baseUrl VARCHAR(256) 
    pattern VARCHAR(256) 
}

Table RWDataRegisterDemand [headercolor: #00B0F0] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    dataset RWDataset 
}

Ref: RWDataRegisterDemand.dataset  > RWDataset.id


Table RWDataRegisterSupply [headercolor: #00B0F0] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    dataset RWDataset 
    status VARCHAR(256) 
}

Ref: RWDataRegisterSupply.dataset  > RWDataset.id


Table RWDataCollectionDemand [headercolor: #00B0F0] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    collectionInfo RWDataCollectionInfo 
}

Ref: RWDataCollectionDemand.collectionInfo  > RWDataCollectionInfo.id


Table RWDataCollectionSupply [headercolor: #00B0F0] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    collectionInfo RWDataCollectionInfo 
    status VARCHAR(256) 
}

Ref: RWDataCollectionSupply.collectionInfo  > RWDataCollectionInfo.id


Table RWDataCollectionStatus [headercolor: #00B0F0] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    collectionInfo RWDataCollectionInfo 
    contentReferenceTime VARCHAR(256) 
    activeConnections JSON 
    trafficStatistics JSON 
}

Ref: RWDataCollectionStatus.collectionInfo  > RWDataCollectionInfo.id


Table RWDataBrewingDemand [headercolor: #C00000] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    brewerInfo RWDataBrewerInfo 
    brewerInput RWDataBrewerInput 
    brewingArgument RWDataBrewingArgument 
    brewerOutputStore RWDataStoringInfo 
    timePeriodStart VARCHAR(256)                    // 読み込むデータがいつからのデータか
    timePeriodEnd VARCHAR(256)                      // 読み込むデータがいつまでのデータか
}

Ref: RWDataBrewingDemand.brewerInfo         > RWDataBrewerInfo.id
Ref: RWDataBrewingDemand.brewerInput        > RWDataBrewerInput.id
Ref: RWDataBrewingDemand.brewingArgument    > RWDataBrewingArgument.id
Ref: RWDataBrewingDemand.brewerOutputStore  > RWDataStoringInfo.id


Table RWDataBrewingSupply [headercolor: #C00000] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    brewerInfo RWDataBrewerInfo 
    brewerOutput RWDataBrewerOutput 
    brewingArgument RWDataBrewingArgument 
    timePeriodStart VARCHAR(256)                    // 読み込むデータがいつからのデータか
    timePeriodEnd VARCHAR(256)                      // 読み込むデータがいつまでのデータか
}

Ref: RWDataBrewingSupply.brewerInfo         > RWDataBrewerInfo.id
Ref: RWDataBrewingSupply.brewerOutput       > RWDataBrewerOutput.id
Ref: RWDataBrewingSupply.brewingArgument    > RWDataBrewingArgument.id


Table RWDataPeriodicBrewingConfig [headercolor: #C00000] {                // RWDB システムが定期的に RWDataBrewingDemand を投げる設定
    id URL [pk]
    name VARCHAR(256)
    url URL
    brewingConfig RWDataBrewingDemand  // 投げる Demand
    cronConfig VARCHAR(256)                         // 時間間隔 (cron format)
}

Ref: RWDataPeriodicBrewingConfig.brewingConfig    > RWDataBrewingDemand.id


Table RWDataReadDemand {
    id URL [pk]
    name VARCHAR(256)
    url URL
    dataset RWDataset                  // 読み込む実世界データセット
    timePeriodStart VARCHAR(256)                    // 読み込むデータがいつからのデータか
    timePeriodEnd VARCHAR(256)                      // 読み込むデータがいつまでのデータか
    sparqlQuery VARCHAR(256)                        // 構造クエリ（SPARQL）
    graphqlQuery VARCHAR(256)                       // 構造クエリ（GraphQL）
}

Ref: RWDataReadDemand.dataset  > RWDataset.id


Table RWDataReadSupply {
    id URL [pk]
    name VARCHAR(256)
    url URL
    dataset RWDataset                  // 読み込む実世界データセット
    timePeriodStart VARCHAR(256)                    // 読み込むデータがいつからのデータか
    timePeriodEnd VARCHAR(256)                      // 読み込むデータがいつまでのデータか
    sparqlQuery VARCHAR(256)                        // 構造クエリ（SPARQL）
    graphqlQuery VARCHAR(256)                       // 構造クエリ（GraphQL）
    data bytes                                // 実際には MBUS で送る？
}

Ref: RWDataReadSupply.dataset  > RWDataset.id


Table RWDataWriteDemand {
    id URL [pk]
    name VARCHAR(256)
    url URL
    dataset RWDataset                  // 書き込む実世界データセット
    timePeriodStart VARCHAR(256)                    // 書き込むデータがいつからのデータか
    timePeriodEnd VARCHAR(256)                      // 書き込むデータがいつまでのデータか
    data bytes                                // 実際には MBUS で送る？
}

Ref: RWDataWriteDemand.dataset  > RWDataset.id


Table RWDataWriteSupply {
    id URL [pk]
    name VARCHAR(256)
    url URL
    dataset RWDataset 
    timePeriodStart VARCHAR(256)                    // 書き込んだデータがいつからのデータか
    timePeriodEnd VARCHAR(256)                      // 書き込んだデータがいつまでのデータか
}

Ref: RWDataWriteSupply.dataset  > RWDataset.id


Table RWDataMoveDemand [headercolor: #2F5597] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    dataset RWDataset                  // 移動する RWDataset
    timePeriodStart VARCHAR(256)                    // 移動する RWDataset のうち対象となるデータの開始時刻はどれだけ前か (ex. 8d → 8日前)
    timePeriodEnd VARCHAR(256)                      // 移動する RWDataset のうち対象となるデータの終了時刻はどれだけ前か (ex. 7d → 7日前)
    moveFrom RWDataStoringInfo         // データの移動元
    moveTo RWDataStoringInfo           // データの移動先
}

Ref: RWDataMoveDemand.dataset  > RWDataset.id
Ref: RWDataMoveDemand.moveFrom > RWDataStoringInfo.id
Ref: RWDataMoveDemand.moveTo   > RWDataStoringInfo.id


Table RWDataMoveSupply [headercolor: #2F5597] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    dataset RWDataset                  // 移動した RWDataset
    timePeriodStart VARCHAR(256)                    // 移動した RWDataset のうち対象となるデータの開始時刻はどれだけ前か (ex. 8d → 8日前)
    timePeriodEnd VARCHAR(256)                      // 移動した RWDataset のうち対象となるデータの終了時刻はどれだけ前か (ex. 7d → 7日前)
    moveFrom RWDataStoringInfo         // データの移動元
    moveTo RWDataStoringInfo           // データの移動先
    movedDataset RWDataset             // 移動先の RWDataset
}

Ref: RWDataMoveSupply.dataset  > RWDataset.id
Ref: RWDataMoveSupply.moveFrom > RWDataStoringInfo.id
Ref: RWDataMoveSupply.moveTo   > RWDataStoringInfo.id
Ref: RWDataMoveSupply.movedDataset  > RWDataset.id


Table RWDataPeriodicMoveConfig [headercolor: #2F5597] {                   // RWDB システムが定期的に RWDataset を移動する設定
    id URL [pk]
    name VARCHAR(256)
    url URL
    moveConfig RWDataMoveDemand        // 移動設定
    cronConfig VARCHAR(256)                       // 時間間隔 (cron format)
}

Ref: RWDataPeriodicMoveConfig.moveConfig  > RWDataMoveDemand.id


Table RWDataRemoveDemand [headercolor: #2F5597] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    datasetStore RWDataStoringInfo     // 削除する RWDataset の RWDataStoringInfo
    timePeriodStart VARCHAR(256)                    // 削除する RWDataset のうち対象となるデータの開始時刻はどれだけ前か (ex. 8d → 8日前)
    timePeriodEnd VARCHAR(256)                      // 削除する RWDataset のうち対象となるデータの終了時刻はどれだけ前か (ex. 7d → 7日前)
}

Ref: RWDataRemoveDemand.datasetStore > RWDataStoringInfo.id


Table RWDataRemoveSupply [headercolor: #2F5597] {
    id URL [pk]
    name VARCHAR(256)
    url URL
    datasetStore RWDataStoringInfo     // 削除した RWDataset の RWDataStoringInfo
    timePeriodStart VARCHAR(256)                    // 削除した RWDataset のうち対象となるデータの開始時刻はどれだけ前か (ex. 8d → 8日前)
    timePeriodEnd VARCHAR(256)                      // 削除した RWDataset のうち対象となるデータの終了時刻はどれだけ前か (ex. 7d → 7日前)
}

Ref: RWDataRemoveSupply.datasetStore > RWDataStoringInfo.id


Table RWDataPeriodicRemoveConfig [headercolor: #2F5597] {                 // RWDB システムが定期的に RWDataset を削除する設定
    id URL [pk]
    name VARCHAR(256)
    url URL
    removeConfig RWDataRemoveDemand    // 削除設定
    cronConfig VARCHAR(256)                       // 時間間隔 (cron format)
}

Ref: RWDataPeriodicRemoveConfig.removeConfig  > RWDataRemoveDemand.id
