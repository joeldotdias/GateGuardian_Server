package com.gateguardian.gateguardianserver.resident.service.impl;

import com.gateguardian.gateguardianserver.resident.model.Resident;
import com.gateguardian.gateguardianserver.resident.repository.ResidentRepository;
import com.gateguardian.gateguardianserver.resident.service.ResidentService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class ResidentServiceImpl implements ResidentService {

   @Autowired
   private ResidentRepository residentRepository;

   @Override
   public Resident getResidentByEmail(String email) {
      return residentRepository.getResidentByEmail(email).get(0);
   }

   @Override
   public void updateResidentPfp(String email, String pfpUrl) {
      Resident updatableResident = residentRepository.getResidentByEmail(email).get(0);
      if(updatableResident  != null) {
         updatableResident.setPfpUrl(pfpUrl);
         residentRepository.save(updatableResident);
      }
   }

   @Override
   public void updateResidentProfile(String email, String name, String aboutMe, String phoneNo) {
      Resident updatableResident = residentRepository.getResidentByEmail(email).get(0);
      if(updatableResident  != null) {
         updatableResident.setName(name);
         updatableResident.setAboutMe(aboutMe);
         updatableResident.setPhoneNo(phoneNo);
         residentRepository.save(updatableResident);
      }
   }
}