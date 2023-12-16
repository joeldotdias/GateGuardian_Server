package com.gateguardian.gateguardianserver.security.service.impl;

import com.gateguardian.gateguardianserver.resident.model.Visitor;
import com.gateguardian.gateguardianserver.resident.repository.ResidentRepository;
import com.gateguardian.gateguardianserver.resident.repository.VisitorRepository;
import com.gateguardian.gateguardianserver.security.SecurityDto;
import com.gateguardian.gateguardianserver.security.model.Security;
import com.gateguardian.gateguardianserver.security.model.VisitorLog;
import com.gateguardian.gateguardianserver.security.repository.SecurityRepository;
import com.gateguardian.gateguardianserver.security.repository.VisitorLogRepository;
import com.gateguardian.gateguardianserver.security.service.SecurityService;
import com.gateguardian.gateguardianserver.user.model.User;
import com.gateguardian.gateguardianserver.user.repository.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;

@Service
public class SecurityServiceImpl implements SecurityService {

   @Autowired
   SecurityRepository securityRepository;

   @Autowired
   UserRepository userRepository;

   @Autowired
   ResidentRepository residentRepository;

   @Autowired
   VisitorRepository visitorRepository;

   @Autowired
   VisitorLogRepository visitorLogRepository;

   @Override
   public void saveSecurity(String name, String email, String adminEmail) {
      String society = residentRepository.getResidentByEmail(adminEmail).get(0).getSociety();
      securityRepository.save(new Security(name, email, society));
      userRepository.save(new User(name, email, "Security", society));
   }

   @Override
   public Security getSecurityByEmail(String email) {
      return securityRepository.getSecurityByEmail(email).get(0);
   }

   @Override
   public List<Visitor> getVisitorsBySociety(String email) {
      Security security = securityRepository.getSecurityByEmail(email).get(0);
      return visitorRepository.getVisitorsBySociety(security.getSociety());
   }

   @Override
   public void moveVerifiedVisitorToLogs(Integer visitorId) {
      Visitor verifiedVisitor = visitorRepository.getVisitorById(visitorId).get(0);
      VisitorLog visitorLog = new VisitorLog(verifiedVisitor.getName(), verifiedVisitor.getPhoneNo(), verifiedVisitor.getHostFlat(), verifiedVisitor.getHostBuilding(), verifiedVisitor.getSociety());
      visitorLogRepository.save(visitorLog);
      visitorRepository.delete(verifiedVisitor);
   }

   @Override
   public List<VisitorLog> getVisitorLogsBySociety(String email) {
      Security security = securityRepository.getSecurityByEmail(email).get(0);
      String society = security.getSociety();
      return visitorLogRepository.getVisitorLogsBySociety(society);
   }

   @Override
   public List<SecurityDto> getSecurityBySociety(String email) {
      String society = residentRepository.getResidentByEmail(email).get(0).getSociety();
      List<Security> securityList = securityRepository.getSecurityBySociety(society);
      List<SecurityDto> securityDtos = new ArrayList<>();

      for(Security security: securityList) {
         SecurityDto securityDto = new SecurityDto(security.getName(), security.getEmail(), security.getBadgeId());
         securityDtos.add(securityDto);
      }

      return securityDtos;
   }

   @Override
   public void updateSecurityPfp(String email, String pfpUrl) {
      Security updatableSecurity = securityRepository.getSecurityByEmail(email).get(0);
      if(updatableSecurity != null) {
         updatableSecurity.setPfpUrl(pfpUrl);
         securityRepository.save(updatableSecurity);
      }
   }

   @Override
   public void updateSecurityProfile(String email, String name, String badgeId, String phoneNo) {
      Security updatableSecurity = securityRepository.getSecurityByEmail(email).get(0);
      if(updatableSecurity != null) {
         updatableSecurity.setName(name);
         updatableSecurity.setBadgeId(badgeId);
         updatableSecurity.setPhoneNo(phoneNo);
         securityRepository.save(updatableSecurity);

         User updatableUser =  userRepository.getUsersByEmail(email).get(0);
         updatableUser.setName(name);
         userRepository.save(updatableUser);
      }
   }
}