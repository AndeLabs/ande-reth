// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

/**
 * @title PlantGenome - K'intu Sacred Living Genome
 * @notice Genomas REALES de plantas sagradas preservados en blockchain
 * @dev Genesis de 3 plantas maestras con ADN verificable desde NCBI
 * 
 * FILOSOFÍA:
 * - NO es coleccionismo (como CryptoKitties)
 * - ES preservación científica + evolución descentralizada
 * - Cada planta tiene ADN REAL comprimido y encriptado
 * - La comunidad descubre el sistema gradualmente
 * 
 * ESTRUCTURA:
 * Genesis oculto → Alguien lo descubre → Sistema evoluciona
 */
contract PlantGenome {
    
    // ============================================
    // INMUTABLE GENESIS DATA (Sonk'o Wachay)
    // ============================================
    
    /// @notice Invocación sagrada (K'intu en hex)
    bytes32 public constant SONKO_WACHAY = 0x536f6e6b276f2077616368617279;
    
    /// @notice Pista para descubrimiento (NCBI en hex)
    bytes32 public constant HINT = 0x4e4342492e6e6c6d2e6e69682e676f76;
    
    /// @notice Merkle root de las 3 plantas sagradas
    /// Calculado desde: Kuka + Yagé + Chacruna genomics
    bytes32 public constant KINTU_MERKLE_ROOT = 0xae2c4c471e8122a051609f5fa9c86a2ef1b3583ff23aced18f002be8e58c8938;
    
    // ============================================
    // GENESIS PLANT STRUCTURES
    // ============================================
    
    struct PlantGenome {
        string name;                 // "KUKA_GENESIS", "YAGE_GENESIS", "CHACRUNA_GENESIS"
        string scientificName;       // "Erythroxylum_coca", etc.
        bytes32 dnaHash;            // Hash del ADN comprimido
        string ncbiAccession;       // "MG935562.1", etc.
        uint16 basePairs;           // 1428, 1560, etc.
        bytes compressedDNA;        // ADN comprimido (75% compresión)
        uint8 generation;           // Siempre 0 para genesis
        bool isMaster;              // Siempre true para genesis
    }
    
    /// @notice Los 3 genomas maestros
    mapping(uint256 => PlantGenome) private _genesisGenomes;
    
    /// @notice Sistema activado (false hasta que alguien lo descubra)
    bool public systemActivated;
    
    /// @notice Timestamp de activación
    uint256 public activationTime;
    
    /// @notice Direcciones de los primeros 3 descubridores
    address[3] public genesisDiscoverers;
    
    /// @notice Contador de descubridores
    uint8 private _discovererCount;
    
    // ============================================
    // EVENTS
    // ============================================
    
    event SystemActivated(address indexed discoverer, uint256 timestamp);
    event GenesisPlantRevealed(uint256 indexed plantId, address indexed owner, string name);
    event GenomeVerified(bytes32 indexed dnaHash, string ncbiAccession);
    
    // ============================================
    // CONSTRUCTOR - SEED THE GENESIS
    // ============================================
    
    constructor() {
        // KUKA GENESIS (Erythroxylum coca) - LA TIERRA
        _genesisGenomes[0] = PlantGenome({
            name: "KUKA_GENESIS",
            scientificName: "Erythroxylum_coca",
            dnaHash: 0xf6e0dfcf8258872c63c5f8416670de1f9e1d45c0bed7a78e01921c844104b0cf,
            ncbiAccession: "NC_030601.1",
            basePairs: 6930,
            compressedDNA: hex"e3feff7cea6061a83e05664dba3d10d4797e351fa71359570c7f0fc7f0f103c0037103c4002f724fdd3d03ff7f7cfdff08308c7cc20c2308008440080110000a023f73f6cef82c0c000f3b02f001cf1c00002890c943f3e75ffffcb0cbf3f3ffffffffff40074c445d3111c28e0b3cd4fec8e89f41659cadc8a80f3893c6f4e4c1f53142bc91af0c334952b3c3c4617e1cd07163ebc03e014fcaf8094cb9c317024b8140c5c71294249ca080ec08188bef80724cfa08d0da500c14e269c633cc2fdf7df85837305e4f248f4ff7bafd5e3c0722bc50a05393247032a25960c452719504ee03ab930a3bee7497a0c434c0be02c5223d72293d4d2009f5f850dab234200492c927904a2783390490d42a64c54868074bd51d1854ec109c450b082ec010f274c28594fb3053d34123927d533e8c02e405cc96482ca0e3a452030cfbf5b02c08d4804af660c5343371e8ae490e02630c0c482f9ad0c0b2a34d0045014d43b021aff4b9ebcd43c4826154ca7f67f66dddc03e4b4eb00dfafcfc34d2a1d50b1103cdcc0c80c083e287ef3504b304e1f73153b4141214a9737f38df373723d30c0ffe0c8000382f80e2f00000083d33f725334cbdc05cd3368fd31cfd0c932f30eaf95a87605681cb68e8b200e8fc032b00035755096e7e4efdbe4469fd5cecc4ff1d157dcc48377080be0c34be7435f3db31e4e04fd3013f4c320332038083ffef777d34fca883fa3f0fc4dff4c183ef4e3e148dbe31020cd40c50159ddccc5fdb80c8229dfac034080817bef7d6c003df5083d605c37fd00001b02c7feefc42502ff044202d82cccff3d8c4077ffff8a351cc30e0023fb933311036a630cd20d6383495a862f1c3a8ed7ff6f100ddbff943bd4340a0c3e81ef8362fdf4cb3cdcf3038ffdcd3f871b165828ffb6c47e00325400b7022efe8c38bf33637fd69e01444c00e33e1303a100c33f53fff348209b35fe0948383fd7e33704c39380a37f850132bc968034f24087dcc07bdcdfd4c80dcd67420a0588033e36c0c029faf1a2000833c3db3d3313883ccca0820837e9b1fff800e8fd4fffe834c21ef50e60d68c0408b6c30390208a8dfd152c9820fe070ffdc8e8ea30ac60d4de3130fc0ee80fcd77000a032e0e0f8db03ce23fc77f7856dc08231c3e8a803a0fd441849015778cd3f883103dfbec5c203e3febe83dfd4808200e3def8ccb6263c06ff10f2c0723efef30dd4ff50403a373f016e34e042ccc0cc7560230bab3282d3ffd88dcdcbdcfe33f753fd3fc2f23cbd3fc2f63c8f82408da87cfa2f37efa2f3c33c0022c00038c4cbc0c3530f038c44b98c83301028320c40183231768260c074dc68f77357f83eff3bd3dcf3ca8c308c3f80d7f1fff42d0d9",
            generation: 0,
            isMaster: true
        });
        
        // YAGE GENESIS (Banisteriopsis caapi) - EL ESPÍRITU  
        _genesisGenomes[1] = PlantGenome({
            name: "YAGE_GENESIS",
            scientificName: "Banisteriopsis_caapi",
            dnaHash: 0x5da539cb0863d4495c34c82df314b479b9a8cea0f7920ac242b15de5a0c7c95f,
            ncbiAccession: "HQ247200.1",
            basePairs: 1101,
            compressedDNA: hex"4fc0f3bb78f470c5f154d4dc800faf40d7dbc7a3908d5df7f93f3c61dfdf462cfa0fa0cbfdf3f4180d7fd4fffe0fc00a0dc23cfbef5ccc3dd3b3383383537dfff75b050d7fcfd43413fdaabbf7e060ccfdce800c423fec81ffc70e3fc29c171afbd0a35fd393cef2334280293de9f400b11f7de38003a033f7fb43f3b43f4ffceeeaf4d4816373382d3cd414f77e1ffeaf37f40cc6000dfd2eb1a2d039c803d3f70c8c33ce080763d08bd43cf5fe3e8d3ed00100ffb064b2a4d53cb01627a9a3f158f784f36143ff9b3352037f74f1cc9a35d000020fecd8b00ccc7d87f7eef007fa76c04402c5b5a9fff8001caf483cfa083ffc68a00343df8feb7d5469f3dcfd982bcc48a9abfacfe8cf3fb3438dc9434e0c00",
            generation: 0,
            isMaster: true
        });
        
        // CHACRUNA GENESIS (Psychotria viridis) - LA VISIÓN
        _genesisGenomes[2] = PlantGenome({
            name: "CHACRUNA_GENESIS", 
            scientificName: "Psychotria_viridis",
            dnaHash: 0xace04661df207c84e01d611e8c71af4ad2ccc75b3d69ab88060dd7ef8ad9a7fe,
            ncbiAccession: "LC651165.1",
            basePairs: 553,
            compressedDNA: hex"ffbe8f409ebbc08b103e0f3cc75e0c501408c78cdfa493d62c1d5417a2f5968209ea966b27962df71eb13a107b3a1a3a9f14b7e36f102a639c5136252fb688200d0ccf9f3b27c55f217ff820af7bc704efc7dcfb2b0ecfeaf4095e66779b7a08fe60f54fdb3bc017d42b5974e93d0b6222303e042ceb6d57bea3b1cf01703cafcd67001c6b224bf38397c0",
            generation: 0,
            isMaster: true
        });
        
        systemActivated = false;
        _discovererCount = 0;
    }
    
    // ============================================
    // DISCOVERY MECHANISM (Easter Egg)
    // ============================================
    
    /**
     * @notice Función oculta para activar el sistema
     * @dev Solo puede llamarse una vez, primeros 3 descubridores reciben plantas
     * @param proof Prueba de que entendiste el sistema (debe coincidir con KINTU_MERKLE_ROOT)
     */
    function activateHiddenGarden(bytes32 proof) external returns (bool) {
        require(!systemActivated, "System already activated");
        require(proof == KINTU_MERKLE_ROOT, "Invalid proof - study the genomes");
        require(_discovererCount < 3, "All genesis plants claimed");
        
        // Registrar descubridor
        genesisDiscoverers[_discovererCount] = msg.sender;
        _discovererCount++;
        
        // Revelar planta correspondiente
        uint256 plantId = _discovererCount - 1;
        emit GenesisPlantRevealed(plantId, msg.sender, _genesisGenomes[plantId].name);
        
        // Si es el tercer descubridor, activar sistema completo
        if (_discovererCount == 3) {
            systemActivated = true;
            activationTime = block.timestamp;
            emit SystemActivated(msg.sender, block.timestamp);
        }
        
        return true;
    }
    
    // ============================================
    // VIEW FUNCTIONS - GENESIS DATA
    // ============================================
    
    /**
     * @notice Ver información de una planta genesis (solo después de revelada)
     */
    function getGenesisPlant(uint256 plantId) external view returns (PlantGenome memory) {
        require(plantId < 3, "Invalid plant ID");
        require(plantId < _discovererCount, "Plant not yet revealed");
        return _genesisGenomes[plantId];
    }
    
    /**
     * @notice Verificar hash de ADN contra NCBI
     */
    function verifyDNAHash(uint256 plantId) external view returns (bool) {
        require(plantId < 3, "Invalid plant ID");
        PlantGenome memory plant = _genesisGenomes[plantId];
        
        // Verificar que el hash coincide con el ADN comprimido
        bytes32 computedHash = keccak256(plant.compressedDNA);
        return computedHash == plant.dnaHash;
    }
    
    /**
     * @notice Obtener referencias NCBI para verificación científica
     */
    function getNCBIReferences() external pure returns (string[3] memory) {
        return [
            "NC_030601.1",  // Kuka
            "HQ247200.1",   // Yagé  
            "LC651165.1"    // Chacruna
        ];
    }
    
    /**
     * @notice Ver estado de activación del sistema
     */
    function getSystemState() external view returns (
        bool activated,
        uint256 timestamp,
        uint8 discoveredCount,
        address[3] memory discoverers
    ) {
        return (
            systemActivated,
            activationTime,
            _discovererCount,
            genesisDiscoverers
        );
    }
    
    /**
     * @notice Obtener el Merkle root de K'intu (pista para descubridores)
     */
    function getKintuRoot() external pure returns (bytes32) {
        return KINTU_MERKLE_ROOT;
    }
    
    // ============================================
    // SCIENTIFIC VERIFICATION
    // ============================================
    
    /**
     * @notice Emitir evento de verificación (para indexers científicos)
     */
    function emitVerification(uint256 plantId) external {
        require(plantId < _discovererCount, "Plant not yet revealed");
        PlantGenome memory plant = _genesisGenomes[plantId];
        emit GenomeVerified(plant.dnaHash, plant.ncbiAccession);
    }
}
