package com.gateguardian.gateguardianserver.resident.controller;

import com.gateguardian.gateguardianserver.resident.dto.ResidentDto;
import com.gateguardian.gateguardianserver.resident.dto.VisitorDto;
import com.gateguardian.gateguardianserver.resident.model.EventMemory;
import com.gateguardian.gateguardianserver.resident.model.Resident;
import com.gateguardian.gateguardianserver.resident.model.Visitor;
import com.gateguardian.gateguardianserver.resident.service.EventMemoryService;
import com.gateguardian.gateguardianserver.resident.service.ResidentService;
import com.gateguardian.gateguardianserver.resident.service.VisitorService;
import com.gateguardian.gateguardianserver.security.SecurityDto;
import com.gateguardian.gateguardianserver.security.service.SecurityService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/resident")
public class ResidentApiController {
   @Autowired
   private ResidentService residentService;

   @Autowired
   private VisitorService visitorService;

   @Autowired
   private EventMemoryService eventMemoryService;

   @Autowired
   private SecurityService securityService;

   @GetMapping("/sign-in")
   public Resident getResidentByEmail(
           @RequestHeader("email") String email
   ) {
      return residentService.getResidentByEmail(email);
   }

   // Profile
   @PutMapping("/update-pfp")
   public void updateResidentPfp(
           @RequestHeader("email") String email,
           @RequestParam("pfpUrl") String pfpUrl
   ) {
      residentService.updateResidentPfp(email, pfpUrl);
   }

   @PutMapping("/update-home")
   public void saveResidentHomeDetails(
           @RequestHeader("email") String email,
           @RequestParam("flat") Integer flat,
           @RequestParam("building") String building
   ) {
      residentService.saveResidentHomeDetails(flat, building, email);
   }

   @PutMapping("/update-profile")
   public void updateResidentProfile(
           @RequestHeader("email") String email,
           @RequestParam("name") String name,
           @RequestParam("aboutMe") String aboutMe,
           @RequestParam("phoneNo") String phoneNo
   ) {
      residentService.updateResidentProfile(email, name, aboutMe, phoneNo);
   }

   @GetMapping("/memories")
   public List<EventMemory> getEventMemoriesByResident(
           @RequestHeader("email") String email
   ) {
      return eventMemoryService.getEventMemoriesByResident(email);
   }

   // Visitors
   @GetMapping("/visitors")
   public List<Visitor> getVisitorsByResidentEmail(
           @RequestHeader("email") String email
   ) {
      return visitorService.getVisitorsByEmail(email);
   }

   @PostMapping("/visitor-save")
   public void saveVisitor(
           @RequestBody VisitorDto visitorDto
   ) {
      visitorService.saveVisitor(visitorDto);
      System.out.println(visitorDto.toString());
   }

   @GetMapping("/visitor-recent")
   public String getRecentVisitorOtp(
           @RequestHeader("email") String email
   ) {
      return visitorService.getRecentVisitorOtp(email);
   }

   @GetMapping("/visitor-otp")
   public String getVisitorOtp(
           @RequestParam("visitorId") Integer visitorId
   ) {
      return visitorService.getVisitorOtpById(visitorId);
   }


   // Admin
   @GetMapping("/admin/residents")
   public List<ResidentDto> getResidentBySociety(
           @RequestHeader("admin") String adminEmail
   ) {
      return residentService.getResidentsBySociety(adminEmail);
   }

   @PostMapping("/admin/save-resident")
   public void saveResident(
           @RequestHeader("admin") String adminEmail,
           @RequestParam("name") String name,
           @RequestParam("email") String email
   ) {
      residentService.saveResident(name, email, adminEmail);
   }

   @GetMapping("/admin/securities")
   public List<SecurityDto> getSecuritiesBySociety(
           @RequestHeader("admin") String adminEmail
   ) {
      return securityService.getSecurityBySociety(adminEmail);
   }

   @PostMapping("/admin/save-security")
   public void saveSecurity(
           @RequestHeader("admin") String adminEmail,
           @RequestParam("name") String name,
           @RequestParam("email") String email
   ) {
      securityService.saveSecurity(name, email, adminEmail);
   }
}