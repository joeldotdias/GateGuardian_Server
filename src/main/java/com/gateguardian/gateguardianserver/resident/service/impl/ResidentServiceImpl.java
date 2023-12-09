package com.gateguardian.gateguardianserver.resident.service.impl;

import com.gateguardian.gateguardianserver.resident.dto.ResidentDto;
import com.gateguardian.gateguardianserver.resident.model.Resident;
import com.gateguardian.gateguardianserver.resident.repository.ResidentRepository;
import com.gateguardian.gateguardianserver.resident.service.ResidentService;
import com.gateguardian.gateguardianserver.user.model.User;
import com.gateguardian.gateguardianserver.user.repository.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;

@Service
public class ResidentServiceImpl implements ResidentService {

   @Autowired
   private ResidentRepository residentRepository;

   @Autowired
   private UserRepository userRepository;

   @Override
   public void saveResident(String name, String email, String adminEmail) {
      String society = residentRepository.getResidentByEmail(adminEmail).get(0).getSociety();
      residentRepository.save(new Resident(name, email, society));
      userRepository.save(new User(name, email, "Resident", society));
   }

   @Override
   public void saveResidentHomeDetails(Integer flatNo, String building, String email) {
      Resident resident = residentRepository.getResidentByEmail(email).get(0);
      resident.setFlatNo(flatNo);
      resident.setBuilding(building);
      residentRepository.save(resident);
   }

   @Override
   public Resident getResidentByEmail(String email) {
      return residentRepository.getResidentByEmail(email).get(0);
   }

   @Override
   public List<ResidentDto> getResidentsBySociety(String email) {
      String society = residentRepository.getResidentByEmail(email).get(0).getSociety();
      List<Resident> residents = residentRepository.getResidentsBySociety(society);
      List<ResidentDto> residentDtos = new ArrayList<>();

      for(Resident resident: residents) {
         ResidentDto residentDto = new ResidentDto(resident.getName(), resident.getEmail(), resident.getFlatNo(), resident.getBuilding());
         residentDtos.add(residentDto);
      }

      return residentDtos;
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

         User updatableUser = userRepository.getUsersByEmail(email).get(0);
         updatableUser.setName(name);
         userRepository.save(updatableUser);
      }
   }
}