package com.gateguardian.gateguardianserver.resident.service;

import com.gateguardian.gateguardianserver.resident.dto.VisitorDto;

import java.util.List;

public interface VisitorService {

   List<VisitorDto> getVisitorsByEmail(String email);

   void saveVisitor(VisitorDto visitorDto);

   String getRecentVisitorOtp(String email);

   String getVisitorOtpById(Integer visitorId);
}