pragma solidity ^0.8.0;

contract DIDRegistry {
    struct DIDDocument {
        string context;
        string id;
        string[] controller;
        string[] alsoKnownAs;
        VerificationMethod[] verificationMethod;
        VerificationMethod[] authentication;
        VerificationMethod[] assertionMethod;
        VerificationMethod[] keyAgreement;
        VerificationMethod[] capabilityInvocation;
        VerificationMethod[] capabilityDelegation;
        ServiceEndpoint[] service;
    }

    struct VerificationMethod {
        string id;
        string type_;
        string publicKey;
    }

    struct ServiceEndpoint {
        string id;
        string type_;
        string serviceEndpoint;
    }

    mapping(string => DIDDocument) public didDocuments;

    function registerDID(string memory did, DIDDocument memory didDoc) public {
        // Verification and storage logic
        didDocuments[did] = didDoc;
    }

    function updateDID(string memory did, DIDDocument memory didDoc) public {
        // Verification and update logic
        didDocuments[did] = didDoc;
    }

    function deactivateDID(string memory did) public {
        // Verification and deactivation logic
        delete didDocuments[did];
    }
}
