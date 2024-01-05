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
           @RequestParam(name = "email") String email
   ) {
      return residentService.getResidentByEmail(email);
   }

   // Profile
   @PutMapping("/update-pfp")
   public void updateResidentPfp(
           @RequestParam(name = "email") String email,
           @RequestParam(name = "pfpUrl") String pfpUrl
   ) {
      residentService.updateResidentPfp(email, pfpUrl);
   }

   @PutMapping("/update-home")
   public void saveResidentHomeDetails(
           @RequestParam(name = "flat") Integer flat,
           @RequestParam(name = "building") String building,
           @RequestParam(name = "email") String email
   ) {
      residentService.saveResidentHomeDetails(flat, building, email);
   }

   @PutMapping("/update-profile")
   public void updateResidentProfile(
           @RequestParam(name = "email") String email,
           @RequestParam(name = "name") String name,
           @RequestParam(name = "aboutMe") String aboutMe,
           @RequestParam(name = "phoneNo") String phoneNo
   ) {
      residentService.updateResidentProfile(email, name, aboutMe, phoneNo);
   }

   @GetMapping("/memories")
   public List<EventMemory> getEventMemoriesByResident(
           @RequestParam(name = "email") String email
   ) {
      return eventMemoryService.getEventMemoriesByResident(email);
   }

   // Visitors
   @GetMapping("/visitors")
   public List<Visitor> getVisitorsByResidentEmail(
           @RequestParam(name = "email") String email
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
           @RequestParam(name = "email") String email
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
           @RequestParam(name = "admin") String adminEmail
   ) {
      return residentService.getResidentsBySociety(adminEmail);
   }

   @PostMapping("/admin/save-resident")
   public void saveResident(
           @RequestParam(name = "name") String name,
           @RequestParam(name = "email") String email,
           @RequestParam(name = "admin") String adminEmail
   ) {
      residentService.saveResident(name, email, adminEmail);
   }

   @GetMapping("/admin/securities")
   public List<SecurityDto> getSecuritiesBySociety(
           @RequestParam(name = "admin") String adminEmail
   ) {
      return securityService.getSecurityBySociety(adminEmail);
   }

   @PostMapping("/admin/save-security")
   public void saveSecurity(
           @RequestParam(name = "name") String name,
           @RequestParam(name = "email") String email,
           @RequestParam(name = "admin") String adminEmail
   ) {
      securityService.saveSecurity(name, email, adminEmail);
   }
}