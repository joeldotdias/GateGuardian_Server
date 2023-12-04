package com.gateguardian.gateguardianserver.resident.service;

import com.gateguardian.gateguardianserver.resident.model.Resident;

public interface ResidentService {

   Resident getResidentByEmail(String email);

   void updateResidentPfp(String email, String pfpUrl);

   void updateResidentProfile(String email, String name, String aboutMe, String phoneNo);
}