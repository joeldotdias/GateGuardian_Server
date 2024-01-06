package com.gateguardian.gateguardianserver.security.controller;

import com.gateguardian.gateguardianserver.resident.model.Visitor;
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
           @RequestHeader("email") String email
   ) {
      return securityService.getSecurityByEmail(email);
   }


   @GetMapping("/visitors")
   public List<Visitor> getVisitorBySociety(
           @RequestHeader("email") String email
   ) {
      return securityService.getVisitorsBySociety(email);
   }

   @DeleteMapping("/visitor-verified")
   public void deleteVerifiedVisitor(
           @RequestParam("id") Integer visitorId
   ) {
      securityService.moveVerifiedVisitorToLogs(visitorId);
   }


   @GetMapping("/visitor-logs")
   public List<VisitorLog> getVisitorLogs(
           @RequestHeader("email") String email
   ) {
      return securityService.getVisitorLogsBySociety(email);
   }


   @PutMapping("/update-pfp")
   public void updateSecurityPfp(
           @RequestHeader("email") String email,
           @RequestParam("pfpUrl") String pfpUrl
   ) {
      securityService.updateSecurityPfp(email, pfpUrl);
   }

   @PutMapping("/update-profile")
   public void updateSecurityProfile(
           @RequestHeader("email") String email,
           @RequestParam("name") String name,
           @RequestParam("badgeId") String badgeId,
           @RequestParam("phoneNo") String phoneNo
   ) {
      securityService.updateSecurityProfile(email, name, badgeId, phoneNo);
   }
}