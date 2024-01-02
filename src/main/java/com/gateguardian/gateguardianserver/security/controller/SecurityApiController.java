package com.gateguardian.gateguardianserver.security.controller;

import com.gateguardian.gateguardianserver.resident.model.Visitor;
import com.gateguardian.gateguardianserver.security.SecurityDto;
import com.gateguardian.gateguardianserver.security.model.Security;
import com.gateguardian.gateguardianserver.security.model.VisitorLog;
import com.gateguardian.gateguardianserver.security.service.SecurityService;
import com.gateguardian.gateguardianserver.security.service.VisitorLogService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/security")
public class SecurityApiController {

   @Autowired
   private SecurityService securityService;

   @Autowired
   private VisitorLogService visitorLogService;

   @GetMapping("/sign-in")
   public Security getSecurityByEmail(
           @RequestParam(name = "email") String email
   ) {
      return securityService.getSecurityByEmail(email);
   }


   @GetMapping("/visitors")
   public List<Visitor> getVisitorBySociety(
           @RequestParam(name = "email") String email
   ) {
      return securityService.getVisitorsBySociety(email);
   }

   @DeleteMapping("/visitor-verified")
   public void deleteVerifiedVisitor(
           @RequestParam(name = "id") Integer visitorId
   ) {
      securityService.moveVerifiedVisitorToLogs(visitorId);
   }


   @GetMapping("/visitor-logs")
   public List<VisitorLog> getVisitorLogs(
           @RequestParam(name = "email") String email
   ) {
      return securityService.getVisitorLogsBySociety(email);
   }


   @PutMapping("/update-pfp")
   public void updateSecurityPfp(
           @RequestParam(name = "email") String email,
           @RequestParam(name = "pfpUrl") String pfpUrl
   ) {
      securityService.updateSecurityPfp(email, pfpUrl);
   }

   @PutMapping("/update-profile")
   public void updateSecurityProfile(
           @RequestParam(name = "email") String email,
           @RequestParam(name = "name") String name,
           @RequestParam(name = "badgeId") String badgeId,
           @RequestParam(name = "phoneNo") String phoneNo
   ) {
      securityService.updateSecurityProfile(email, name, badgeId, phoneNo);
   }

   @GetMapping("/securities")
   public List<SecurityDto> getSecuritiesBySociety(
           @RequestParam(name = "admin") String adminEmail
   ) {
      return securityService.getSecurityBySociety(adminEmail);
   }

   @PostMapping("/save")
   public void saveSecurity(
           @RequestParam(name = "name") String name,
           @RequestParam(name = "email") String email,
           @RequestParam(name = "admin") String adminEmail
   ) {
      securityService.saveSecurity(name, email, adminEmail);
   }
}