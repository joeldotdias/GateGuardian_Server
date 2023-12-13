package com.gateguardian.gateguardianserver.resident.service;

import com.gateguardian.gateguardianserver.resident.dto.VisitorCredDto;
import com.gateguardian.gateguardianserver.resident.dto.VisitorDto;
import com.gateguardian.gateguardianserver.resident.model.Visitor;

import java.util.List;

public interface VisitorService {

   List<Visitor> getVisitorsByEmail(String email);

   void saveVisitor(VisitorDto visitorDto);

   String getRecentVisitorOtp(String email);
   VisitorCredDto getRecentVisitorCred(String email);

   String getVisitorOtpById(Integer visitorId);
}