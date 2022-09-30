window.SIDEBAR_ITEMS = {"constant":[["AUTH_FIREWALL_DROP","IPsec dropped an incoming ESP packet in authenticated firewall mode. This drop is benign."],["BAD_SPI","The SPI in the packet does not match a valid IPsec SA."],["CLEAR_TEXT_DROP","IPsec dropped a clear text packet."],["DEFAULT_MM_AUTH_NOT_FOUND","The specified default main mode authentication list was not found."],["DEFAULT_MM_POLICY_NOT_FOUND","The specified default main mode policy was not found."],["DEFAULT_QM_POLICY_NOT_FOUND","The specified default quick mode policy was not found."],["DOSP_BLOCK","IPsec DoS Protection matched an explicit block rule."],["DOSP_INVALID_PACKET","IPsec DoS Protection received an incorrectly formatted packet."],["DOSP_KEYMOD_NOT_ALLOWED","IPsec DoS Protection received an IPsec negotiation packet for a keying module which is not allowed by policy."],["DOSP_MAX_ENTRIES","IPsec DoS Protection failed to create state because the maximum number of entries allowed by policy has been reached."],["DOSP_MAX_PER_IP_RATELIMIT_QUEUES","IPsec DoS Protection failed to create a per internal IP rate limit queue because the maximum number of queues allowed by policy has been reached."],["DOSP_NOT_INSTALLED","IPsec DoS Protection has not been enabled."],["DOSP_RECEIVED_MULTICAST","IPsec DoS Protection received an IPsec specific multicast packet which is not allowed."],["DOSP_STATE_LOOKUP_FAILED","IPsec DoS Protection failed to look up state."],["IKE_ADD_UPDATE_KEY_FAILED","Failed to add Security Association to IPsec Driver. The most common cause for this is if the IKE negotiation took too long to complete. If the problem persists, reduce the load on the faulting machine."],["IKE_ATTRIB_FAIL","IKE security attributes are unacceptable"],["IKE_AUTHORIZATION_FAILURE","SA establishment is not authorized."],["IKE_AUTHORIZATION_FAILURE_WITH_OPTIONAL_RETRY","SA establishment is not authorized.  You may need to enter updated or different credentials such as a smartcard."],["IKE_AUTH_FAIL","IKE authentication credentials are unacceptable"],["IKE_BENIGN_REINIT","Temporary state created to perform reinitialization. This is not a real failure."],["IKE_CERT_CHAIN_POLICY_MISMATCH","Certificate doesn’t chain to a trusted root in IPsec policy."],["IKE_CGA_AUTH_FAILED","Could not verify binding between CGA address and certificate."],["IKE_COEXISTENCE_SUPPRESS","SA was deleted due to IKEv1/AuthIP co-existence suppress check."],["IKE_CRITICAL_PAYLOAD_NOT_RECOGNIZED","Don’t know how to process critical payload"],["IKE_CRL_FAILED","Certificate Revocation Check failed"],["IKE_DECRYPT","Error decrypting payload"],["IKE_DH_FAIL","Failure in Diffie-Hellman computation"],["IKE_DH_FAILURE","Diffie-Hellman failure"],["IKE_DOS_COOKIE_SENT","Sent DoS cookie notify to initiator."],["IKE_DROP_NO_RESPONSE","No response from peer"],["IKE_ENCRYPT","Error encrypting payload"],["IKE_ERROR","Unknown error occurred"],["IKE_FAILQUERYSSP","Failed to query Kerberos package to obtain max token size."],["IKE_FAILSSPINIT","Failed to obtain security function table dispatch address from SSPI."],["IKE_GENERAL_PROCESSING_ERROR","General processing error"],["IKE_GETSPIFAIL","Failed to obtain new SPI for the inbound SA from IPsec driver. The most common cause for this is that the driver does not have the correct filter. Check your policy to verify the filters."],["IKE_INNER_IP_ASSIGNMENT_FAILURE","Error in assigning inner IP address to initiator in tunnel mode."],["IKE_INVALID_AUTH_ALG","Invalid authentication algorithm"],["IKE_INVALID_AUTH_PAYLOAD","Received invalid authentication offers."],["IKE_INVALID_CERT_KEYLEN","Key length in certificate is too small for configured security requirements."],["IKE_INVALID_CERT_TYPE","Invalid certificate type"],["IKE_INVALID_COOKIE","Invalid cookie received."],["IKE_INVALID_ENCRYPT_ALG","Invalid encryption algorithm"],["IKE_INVALID_FILTER","Given filter is invalid"],["IKE_INVALID_GROUP","Invalid Diffie-Hellman group"],["IKE_INVALID_HASH","Hash verification failed"],["IKE_INVALID_HASH_ALG","Invalid hash algorithm"],["IKE_INVALID_HASH_SIZE","Invalid hash size"],["IKE_INVALID_HEADER","Invalid header"],["IKE_INVALID_KEY_USAGE","Invalid certificate key usage"],["IKE_INVALID_MAJOR_VERSION","The recipient cannot handle version of IKE specified in the header."],["IKE_INVALID_MM_FOR_QM","Parameters of the main mode are invalid for this quick mode."],["IKE_INVALID_PAYLOAD","Invalid payload received"],["IKE_INVALID_POLICY","Invalid policy"],["IKE_INVALID_RESPONDER_LIFETIME_NOTIFY","The lifetime value received in the Responder Lifetime Notify is below the Windows 2000 configured minimum value. Please fix the policy on the peer machine."],["IKE_INVALID_SIG","Invalid certificate signature"],["IKE_INVALID_SIGNATURE","Failed to verify signature"],["IKE_INVALID_SITUATION","Invalid situation"],["IKE_KERBEROS_ERROR","Failed to authenticate using Kerberos"],["IKE_KILL_DUMMY_NAP_TUNNEL","NAP reauth succeeded and must delete the dummy NAP IKEv2 tunnel."],["IKE_LOAD_FAILED","Load failed"],["IKE_LOAD_SOFT_SA","Soft SA loaded"],["IKE_MM_ACQUIRE_DROP","Negotiation request sat in Queue too long"],["IKE_MM_DELAY_DROP","Negotiation took too long"],["IKE_MM_EXPIRED","Main mode SA lifetime expired or peer sent a main mode delete."],["IKE_MM_LIMIT","Max number of established MM SAs to peer exceeded."],["IKE_NEGOTIATION_DISABLED","IKE received a policy that disables negotiation."],["IKE_NEGOTIATION_PENDING","IKE Negotiation in progress"],["IKE_NEG_STATUS_BEGIN","ERROR_IPSEC_IKE_NEG_STATUS_BEGIN"],["IKE_NOTCBPRIV","Failed to enabled TCB privilege."],["IKE_NO_CERT","IKE failed to find valid machine certificate. Contact your Network Security Administrator about installing a valid certificate in the appropriate Certificate Store."],["IKE_NO_MM_POLICY","There is no available Main Mode IKE policy."],["IKE_NO_PEER_CERT","Peer failed to send valid machine certificate"],["IKE_NO_POLICY","No policy configured"],["IKE_NO_PRIVATE_KEY","IKE negotiation failed because the machine certificate used does not have a private key. IPsec certificates require a private key. Contact your Network Security administrator about replacing with a certificate that has a private key."],["IKE_NO_PUBLIC_KEY","Peer’s certificate did not have a public key"],["IKE_OUT_OF_MEMORY","Memory allocation failed."],["IKE_PEER_CRL_FAILED","Certification Revocation check of peer’s certificate failed"],["IKE_PEER_DOESNT_SUPPORT_MOBIKE","Peer does not support MOBIKE."],["IKE_PEER_MM_ASSUMED_INVALID","Main mode SA assumed to be invalid because peer stopped responding."],["IKE_POLICY_CHANGE","New policy invalidated SAs formed with old policy"],["IKE_POLICY_MATCH","Policy match error"],["IKE_PROCESS_ERR","Error processing error payload"],["IKE_PROCESS_ERR_CERT","Error processing Cert payload"],["IKE_PROCESS_ERR_CERT_REQ","Error processing Certificate Request payload"],["IKE_PROCESS_ERR_DELETE","Error processing Delete Payload"],["IKE_PROCESS_ERR_HASH","Error processing Hash payload"],["IKE_PROCESS_ERR_ID","Error processing ID payload"],["IKE_PROCESS_ERR_KE","Error processing KE payload"],["IKE_PROCESS_ERR_NATOA","Error processing NatOA payload."],["IKE_PROCESS_ERR_NONCE","Error processing Nonce payload"],["IKE_PROCESS_ERR_NOTIFY","Error processing Notify payload"],["IKE_PROCESS_ERR_PROP","Error processing Proposal payload"],["IKE_PROCESS_ERR_SA","Error processing SA payload"],["IKE_PROCESS_ERR_SIG","Error processing Signature payload"],["IKE_PROCESS_ERR_TRANS","Error processing Transform payload"],["IKE_PROCESS_ERR_VENDOR","Error processing VendorId payload"],["IKE_QM_ACQUIRE_DROP","Negotiation request sat in Queue too long"],["IKE_QM_DELAY_DROP","Negotiation took too long"],["IKE_QM_EXPIRED","Quick mode SA was expired by IPsec driver."],["IKE_QM_LIMIT","Reached maximum quick mode limit for the main mode. New main mode will be started."],["IKE_QUEUE_DROP_MM","Negotiation request sat in Queue too long"],["IKE_QUEUE_DROP_NO_MM","Negotiation request sat in Queue too long"],["IKE_RATELIMIT_DROP","Incoming SA request was dropped due to peer IP address rate limiting."],["IKE_REQUIRE_CP_PAYLOAD_MISSING","Require configuration payload missing."],["IKE_RPC_DELETE","Deleted via RPC call"],["IKE_SA_DELETED","IKE SA deleted by peer before establishment completed"],["IKE_SA_REAPED","IKE SA deleted before establishment completed"],["IKE_SECLOADFAIL","Failed to load SECURITY.DLL."],["IKE_SHUTTING_DOWN","IKE service is shutting down."],["IKE_SIMULTANEOUS_REKEY","Simultaneous rekeys were detected."],["IKE_SOFT_SA_TORN_DOWN","Soft SA torn down"],["IKE_SRVACQFAIL","Failed to obtain Kerberos server credentials for ISAKMP/ERROR_IPSEC_IKE service. Kerberos authentication will not function. The most likely reason for this is lack of domain membership. This is normal if your computer is a member of a workgroup."],["IKE_SRVQUERYCRED","Failed to determine SSPI principal name for ISAKMP/ERROR_IPSEC_IKE service (QueryCredentialsAttributes)."],["IKE_STRONG_CRED_AUTHORIZATION_AND_CERTMAP_FAILURE","SA establishment is not authorized because there is not a sufficiently strong PKINIT-based credential. This might be related to certificate-to-account mapping failure for the SA."],["IKE_STRONG_CRED_AUTHORIZATION_FAILURE","SA establishment is not authorized because there is not a sufficiently strong PKINIT-based credential."],["IKE_TIMED_OUT","Negotiation timed out"],["IKE_TOO_MANY_FILTERS","Too many dynamically added IKEEXT filters were detected."],["IKE_UNEXPECTED_MESSAGE_ID","Received unexpected message ID."],["IKE_UNKNOWN_DOI","Invalid DOI"],["IKE_UNSUPPORTED_ID","Unsupported ID"],["INTEGRITY_CHECK_FAILED","IPsec integrity check failed."],["INVALID_PACKET","IPsec header and/or trailer in the packet is invalid."],["KEY_MODULE_IMPERSONATION_NEGOTIATION_PENDING","A negotiation running as the security principle who issued the connection is in progress"],["MM_AUTH_EXISTS","The specified main mode authentication list exists."],["MM_AUTH_IN_USE","The specified main mode authentication list is being used."],["MM_AUTH_NOT_FOUND","The specified main mode authentication list was not found."],["MM_AUTH_PENDING_DELETION","The Main Mode authentication bundle is pending deletion."],["MM_FILTER_EXISTS","The specified main mode filter already exists."],["MM_FILTER_NOT_FOUND","The specified main mode filter was not found."],["MM_FILTER_PENDING_DELETION","The Main Mode filter is pending deletion."],["MM_POLICY_EXISTS","The specified main mode policy already exists."],["MM_POLICY_IN_USE","The specified main mode policy is being used."],["MM_POLICY_NOT_FOUND","The specified main mode policy was not found"],["MM_POLICY_PENDING_DELETION","The Main Mode policy is pending deletion."],["QM_POLICY_EXISTS","The specified quick mode policy already exists."],["QM_POLICY_IN_USE","The specified quick mode policy is being used."],["QM_POLICY_NOT_FOUND","The specified quick mode policy was not found."],["QM_POLICY_PENDING_DELETION","The Quick Mode policy is pending deletion."],["REPLAY_CHECK_FAILED","Packet sequence number replay check failed."],["SA_LIFETIME_EXPIRED","Packet was received on an IPsec SA whose lifetime has expired."],["SERVICE_STOPPED",""],["THROTTLE_DROP","IPsec dropped a packet due to DoS throttling."],["TRANSPORT_FILTER_EXISTS","The specified transport mode filter already exists."],["TRANSPORT_FILTER_NOT_FOUND","The specified transport mode filter does not exist."],["TRANSPORT_FILTER_PENDING_DELETION","The transport filter is pending deletion."],["TUNNEL_FILTER_EXISTS","The specified tunnel mode filter exists."],["TUNNEL_FILTER_NOT_FOUND","The specified tunnel mode filter was not found."],["TUNNEL_FILTER_PENDING_DELETION","The tunnel filter is pending deletion."],["WRONG_SA","Packet was received on an IPsec SA that does not match the packet characteristics."]]};