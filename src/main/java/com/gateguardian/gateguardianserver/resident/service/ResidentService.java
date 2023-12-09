package com.gateguardian.gateguardianserver.resident.service;

import com.gateguardian.gateguardianserver.resident.dto.ResidentDto;
import com.gateguardian.gateguardianserver.resident.model.Resident;

import java.util.List;

public interface ResidentService {

   void saveResident(String name, String email, String adminEmail);

   void saveResidentHomeDetails(Integer flatNo, String building, String email);

   Resident getResidentByEmail(String email);

   List<ResidentDto> getResidentsBySociety(String society);

   void updateResidentPfp(String email, String pfpUrl);

   void updateResidentProfile(String email, String name, String aboutMe, String phoneNo);
}