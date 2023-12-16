package com.gateguardian.gateguardianserver.security.service;

import com.gateguardian.gateguardianserver.resident.model.Visitor;
import com.gateguardian.gateguardianserver.security.SecurityDto;
import com.gateguardian.gateguardianserver.security.model.Security;
import com.gateguardian.gateguardianserver.security.model.VisitorLog;

import java.util.List;

public interface SecurityService {

   void saveSecurity(String name, String email, String adminEmail);

   Security getSecurityByEmail(String email);

   List<Visitor> getVisitorsBySociety(String email);

   void moveVerifiedVisitorToLogs(Integer visitorId);

   List<VisitorLog> getVisitorLogsBySociety(String email);

   List<SecurityDto> getSecurityBySociety(String email);

   void updateSecurityPfp(String email, String pfpUrl);

   void updateSecurityProfile(String email, String name, String badgeId, String phoneNo);
}