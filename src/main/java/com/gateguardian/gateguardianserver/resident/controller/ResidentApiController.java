package com.gateguardian.gateguardianserver.resident.controller;

import com.gateguardian.gateguardianserver.resident.dto.ResidentDto;
import com.gateguardian.gateguardianserver.resident.dto.VisitorCredDto;
import com.gateguardian.gateguardianserver.resident.dto.VisitorDto;
import com.gateguardian.gateguardianserver.resident.model.EventMemory;
import com.gateguardian.gateguardianserver.resident.model.Resident;
import com.gateguardian.gateguardianserver.resident.model.Visitor;
import com.gateguardian.gateguardianserver.resident.service.EventMemoryService;
import com.gateguardian.gateguardianserver.resident.service.ResidentService;
import com.gateguardian.gateguardianserver.resident.service.VisitorService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
public class ResidentApiController {
   @Autowired
   private ResidentService residentService;

   @Autowired
   private VisitorService visitorService;

   @Autowired
   private EventMemoryService eventMemoryService;

   @GetMapping("/resident")
   public Resident getResidentByEmail(
           @RequestParam(name = "email") String email
   ) {
      return residentService.getResidentByEmail(email);
   }

   @GetMapping("/residents")
   public List<ResidentDto> getResidentBySociety(
           @RequestParam(name = "admin") String adminEmail
   ) {
      return residentService.getResidentsBySociety(adminEmail);
   }

   @PutMapping("/update-pfp")
   public void updateResidentPfp(
           @RequestParam(name = "email") String email,
           @RequestParam(name = "pfpUrl") String pfpUrl
   ) {
      residentService.updateResidentPfp(email, pfpUrl);
   }

   @PostMapping("/resident-save")
   public void saveResident(
           @RequestParam(name = "name") String name,
           @RequestParam(name = "email") String email,
           @RequestParam(name = "admin") String adminEmail
   ) {
      residentService.saveResident(name, email, adminEmail);
   }

   @PostMapping("/resident-home")
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
   }

   @GetMapping("/visitor-recent")
   public String getRecentVisitorOtp(
           @RequestParam(name = "email") String email
   ) {
      return visitorService.getRecentVisitorOtp(email);
   }

//   @GetMapping("/visitor-recento")
//   public VisitorCredDto getRecentVisitorOtpO(
//           @RequestParam(name = "email") String email
//   ) {
//      return visitorService.getRecentVisitorCred(email);
//   }

   @GetMapping("/visitor-otp")
   public String getVisitorOtp(
           @RequestParam("visitorId") Integer visitorId
   ) {
      return visitorService.getVisitorOtpById(visitorId);
   }


   @GetMapping("/resident-memories")
   public List<EventMemory> getEventMemoriesByResident(
           @RequestParam(name = "email") String email
   ) {
      return eventMemoryService.getEventMemoriesByResident(email);
   }
}