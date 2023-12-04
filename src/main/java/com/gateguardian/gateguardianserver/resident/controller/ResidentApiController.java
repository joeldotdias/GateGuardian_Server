package com.gateguardian.gateguardianserver.resident.controller;

import com.gateguardian.gateguardianserver.resident.model.Resident;
import com.gateguardian.gateguardianserver.resident.service.ResidentService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class ResidentApiController {
   @Autowired
   private ResidentService residentService;


   @GetMapping("/resident")
   public Resident getResidentByEmail(
           @RequestParam(name = "email") String email
   ) {
      return residentService.getResidentByEmail(email);
   }

   @PutMapping("/update-pfp")
   public void updateResidentPfp(
           @RequestParam(name = "email") String email,
           @RequestParam(name = "pfpUrl") String pfpUrl
   ) {
      residentService.updateResidentPfp(email, pfpUrl);
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
}