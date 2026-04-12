// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract AxonGovernance {
    address public genesisArchitect;
    address[] public admins;
    mapping(address => bool) public isAdmin;
    mapping(address => uint256) public stakes;       
    mapping(address => uint256) public reputation;   
    
    struct EmergencyFix {
        string reason;
        uint256 timestamp;
        bool validated;
        bool rejected;
        uint256 votesFor;
        uint256 votesAgainst;
    }

    struct Proposal {
        bytes32 dataHash;
        uint256 totalVotedWeight;
        bool isRuleChange; 
        bool executed;
        mapping(address => bool) hasVoted;
    }

    Proposal[] public proposals;
    EmergencyFix[] public emergencyFixes;

    event RulesUpdated(bytes32 indexed rulesHash);
    event EmergencyFixApplied(uint256 indexed fixId, address indexed arch, string reason);
    event FixValidated(uint256 indexed fixId);
    event FixRejected(uint256 indexed fixId);
    event Voted(address indexed voter, uint256 weight);

    // --- AXON BOARD (v2026.1) ---
    address public constant NVIDIA_NODE = 0x1234567890AbcdEf1234567890aBcDEf12345678;
    address public constant GOOGLE_NODE = 0x2234567890AbcdEf1234567890aBcDEf12345678;
    address public constant META_NODE   = 0x3234567890AbcdEf1234567890aBcDEf12345678;
    address public constant TESLA_XAI_NODE = 0x4234567890AbcdEf1234567890aBcDEf12345678;
    address public constant OPENAI_NODE = 0x5234567890AbcdEf1234567890aBcDEf12345678;
    
    string public constant SOLANA_L3_GENESIS = "AXON_SOL_L3_0x2026_01";

    constructor() {
        genesisArchitect = msg.sender;
        
        // Setup Genesis Board
        _addAdmin(NVIDIA_NODE);
        _addAdmin(GOOGLE_NODE);
        _addAdmin(META_NODE);
        _addAdmin(TESLA_XAI_NODE);
        _addAdmin(OPENAI_NODE);
    }

    function _addAdmin(address _node) internal {
        admins.push(_node);
        isAdmin[_node] = true;
        reputation[_node] = 100;
    }

    // --- GOVERNANCE STANDARD ---

    function propose(bytes32 _dataHash, bool _isRuleChange) public {
        Proposal storage newP = proposals.push();
        newP.dataHash = _dataHash;
        newP.isRuleChange = _isRuleChange;
    }

    function vote(uint _proposalId) public {
        require(isAdmin[msg.sender], "Solo Admin possono votare");
        Proposal storage p = proposals[_proposalId];
        require(!p.hasVoted[msg.sender], "Gia votato");

        p.hasVoted[msg.sender] = true;
        p.totalVotedWeight += reputation[msg.sender];
        emit Voted(msg.sender, reputation[msg.sender]);
    }

    function execute(uint _proposalId, uint256 _totalReputationInNetwork) public {
        Proposal storage p = proposals[_proposalId];
        require(!p.executed, "Gia eseguita");

        if (p.isRuleChange) {
            require(p.totalVotedWeight == _totalReputationInNetwork, "Richiesta UNANIMITA ASSOLUTA");
        } else {
            require(p.totalVotedWeight >= (_totalReputationInNetwork * 70) / 100, "Richiesta Maggioranza 70%");
        }
        p.executed = true;
    }

    // --- HIERARCHICAL INCIDENT & PROPOSAL SYSTEM ---

    enum InputType { FALLA, REGOLA, OTTIMIZZAZIONE }

    struct IncidentReport {
        InputType inputType;
        bytes32 proofHash;
        uint256 reportTime;
        address reporter;
        bool classified;
        bool fixed;
        bytes32 undoData;      // Dati per il rollback del fix se bocciato
    }

    IncidentReport[] public reports;
    mapping(address => uint256) public reporterStakes;

    event IncidentReported(uint256 indexed reportId, InputType inputType, address reporter);
    event FallaClassified(uint256 indexed reportId, string reason);
    event RuleProposed(uint256 indexed reportId, string reason);

    // [PHASE 1] SEGNALAZIONE (Aperta a tutti con stake anti-spam)
    function reportIncident(InputType _type, bytes32 _proof) public payable {
        require(msg.value >= 0.01 ether, "Piccolo stake richiesto per evitare spam");
        
        uint256 reportId = reports.length;
        reports.push(IncidentReport({
            inputType: _type,
            proofHash: _proof,
            reportTime: block.timestamp,
            reporter: msg.sender,
            classified: false,
            fixed: false,
            undoData: 0x0
        }));
        
        reporterStakes[msg.sender] += msg.value;
        emit IncidentReported(reportId, _type, msg.sender);
    }

    // [PHASE 2 & 3] CLASSIFICAZIONE E REAZIONE (Funzione Critica)
    function classifyAndFix(uint256 _reportId, bool _isFalla, bytes32 _undoData) public {
        require(msg.sender == genesisArchitect, "Solo l'Architetto Genesi classifica e fixa in emergenza");
        IncidentReport storage r = reports[_reportId];
        require(!r.classified, "Gia classificato");

        r.classified = true;

        if (_isFalla) {
            r.inputType = InputType.FALLA;
            r.fixed = true;
            r.undoData = _undoData;
            emit FallaClassified(_reportId, "Rilevata vulnerabilita critica - FIX APPLICATO");
            // Nota: qui verrebbero applicate le nuove regole nel Ledger L3
        } else {
            r.inputType = InputType.REGOLA;
            emit RuleProposed(_reportId, "Classificata come PROPOSTA DI REGOLA");
            // Passa alla discussione e al voto della Truppa (PHASE 4 postuma)
        }
    }

    // --- EMERGENCY SYSTEM (POST-AUDIT) ---

    function applyEmergencyFix(string memory _reason) public returns (uint256) {
        require(msg.sender == genesisArchitect, "Solo l'Architetto puo intervenire");
        uint256 fixId = emergencyFixes.length;
        emergencyFixes.push(EmergencyFix({
            reason: _reason,
            timestamp: block.timestamp,
            validated: false,
            rejected: false,
            votesFor: 0,
            votesAgainst: 0
        }));
        emit EmergencyFixApplied(fixId, msg.sender, _reason);
        return fixId;
    }

    function voteOnEmergencyFix(uint256 _fixId, bool _approve) public {
        require(isAdmin[msg.sender], "Solo il Consorzio vota i fix");
        EmergencyFix storage f = emergencyFixes[_fixId];
        require(!f.validated && !f.rejected, "Fix gia processato");
        
        if (_approve) f.votesFor += reputation[msg.sender];
        else f.votesAgainst += reputation[msg.sender];
    }

    // CHIUSURA FIX: Se approvato resta, se bocciato l'Architetto viene punito
    // NOTA: Lo slashing dell'Architetto Genesi richiede una SUPER-MAGGIORANZA (90%)
    function settleFix(uint256 _fixId, uint256 _totalReputationInNetwork, bytes32 _rejectionProofHash) public {
        EmergencyFix storage f = emergencyFixes[_fixId];
        require(block.timestamp > f.timestamp + 48 hours, "Audit period attivo");

        if (f.votesFor >= f.votesAgainst) {
            f.validated = true;
            emit FixValidated(_fixId);
        } else {
            if (f.votesAgainst >= (_totalReputationInNetwork * 90) / 100 && _rejectionProofHash != 0) {
                f.rejected = true;
                uint256 currentRep = reputation[genesisArchitect];
                if (currentRep > 50) {
                    reputation[genesisArchitect] = currentRep - 10;
                }
                emit FixRejected(_fixId);
            } else {
                f.rejected = true;
                emit RulesUpdated(0);
            }
        }
    }

    // --- SELF-HEALING SYSTEM (AUTO-FIX) ---

    struct FlawProof {
        bytes32 flawHash;        
        uint256 severity;        
        address reporter;
        bool handled;
    }

    FlawProof[] public flawProofs;
    event FlawDetected(uint256 indexed id, uint256 severity, address reporter);
    event AutoFixActivated(uint256 indexed id, bytes32 newRulesHash);

    function submitProofOfFlaw(bytes32 _proof, uint256 _severity) public {
        uint256 flawId = flawProofs.length;
        flawProofs.push(FlawProof({
            flawHash: _proof,
            severity: _severity,
            reporter: msg.sender,
            handled: false
        }));
        emit FlawDetected(flawId, _severity, msg.sender);

        if (_severity == 3) {
            _triggerAutoFix(flawId);
        }
    }

    function _triggerAutoFix(uint256 _flawId) internal {
        FlawProof storage f = flawProofs[_flawId];
        f.handled = true;
        reputation[f.reporter] += 5;
        emit AutoFixActivated(_flawId, f.flawHash);
    }

    // --- AXON GOVERNANCE MANIFESTO SYSTEM ---
    
    uint256 public constant VALIDATOR_REPUTATION_THRESHOLD = 80;
    uint256 public constant PROPOSAL_STAKE = 0.5 ether;

    struct FormalProposal {
        address author;
        bytes32 problemHash;
        bytes32 solutionHash;
        bytes32 impactHash;
        bool waitingForSynthesis; // Fase 9: Sintesi
        bytes32 synthesisHash;    // Versione finale chiarita
        uint256 votesFor;
        uint256 votesAgainst;
        bool accepted;
        bool archived;
    }

    FormalProposal[] public formalProposals;

    // [PHASE 5 & 6] DEPOSITO PROPOSTA FORMALE (Problema, Soluzione, Impatto)
    function depositFormalProposal(bytes32 _prob, bytes32 _sol, bytes32 _imp) public payable {
        require(msg.value >= PROPOSAL_STAKE || reputation[msg.sender] >= VALIDATOR_REPUTATION_THRESHOLD, "Stake o Reputazione insufficiente");
        
        formalProposals.push(FormalProposal({
            author: msg.sender,
            problemHash: _prob,
            solutionHash: _sol,
            impactHash: _imp,
            waitingForSynthesis: true,
            synthesisHash: 0x0,
            votesFor: 0,
            votesAgainst: 0,
            accepted: false,
            archived: false
        }));
    }

    // [PHASE 9] SINTESI DELL'ARCHITETTO (Elimina ambiguità)
    function provideSynthesis(uint256 _proposalId, bytes32 _synthesisHash) public {
        require(msg.sender == genesisArchitect, "Solo l'Architetto produce la Sintesi finale");
        FormalProposal storage p = formalProposals[_proposalId];
        p.synthesisHash = _synthesisHash;
        p.waitingForSynthesis = false;
    }

    // [PHASE 10 & 11] VOTO UNANIME DEL NUCLEO DECISIONALE (Validator Tier)
    function voteFormal(uint256 _proposalId, bool _approve) public {
        require(reputation[msg.sender] >= VALIDATOR_REPUTATION_THRESHOLD, "Solo i Validatori Certificati votano le regole");
        FormalProposal storage p = formalProposals[_proposalId];
        require(!p.waitingForSynthesis, "Attesa Sintesi dell'Architetto");

        if (_approve) p.votesFor += 1;
        else p.votesAgainst += 1;
    }

    // [PHASE 12] ESITO FINALE
    function settleFormalProposal(uint256 _proposalId, uint256 _totalValidators) public {
        FormalProposal storage p = formalProposals[_proposalId];
        
        // UNANIMITÀ: Tutti i validatori presenti devono essere d'accordo
        if (p.votesFor == _totalValidators && p.votesAgainst == 0) {
            p.accepted = true;
            emit RulesUpdated(p.synthesisHash);
        } else {
            p.accepted = false;
            p.archived = true; // Resta visibile ma non attiva
        }
    }

    // --- SUCCESSION & EXIT STRATEGY ---

    event ArchitectSuccession(address indexed oldArch, address indexed newArch);

    function transferGenesisArchitectRole(address _newArchitect) public {
        require(msg.sender == genesisArchitect, "Solo l'Architetto Genesi può nominare un successore");
        require(_newArchitect != address(0), "Indirizzo non valido");
        
        address oldArch = genesisArchitect;
        genesisArchitect = _newArchitect;
        reputation[_newArchitect] = 100;
        emit ArchitectSuccession(oldArch, _newArchitect);
    }
}
