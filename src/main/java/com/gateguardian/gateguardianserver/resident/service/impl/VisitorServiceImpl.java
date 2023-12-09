package com.gateguardian.gateguardianserver.resident.service.impl;

import com.gateguardian.gateguardianserver.resident.dto.VisitorDto;
import com.gateguardian.gateguardianserver.resident.model.Visitor;
import com.gateguardian.gateguardianserver.resident.repository.VisitorRepository;
import com.gateguardian.gateguardianserver.resident.service.VisitorService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.List;

@Service
public class VisitorServiceImpl implements VisitorService {

   @Autowired
   private VisitorRepository visitorRepository;

   @Override
   public List<VisitorDto> getVisitorsByEmail(String email) {
      List<Visitor> visitors = visitorRepository.getVisitorsByEmail(email);
      List<VisitorDto> visitorDtos = new ArrayList<>();

      for(Visitor visitor: visitors) {
         VisitorDto visitorDto = new VisitorDto(visitor.getVisitorId(), visitor.getName(), visitor.getPhoneNo(), visitor.getResidentEmail());
         visitorDtos.add(visitorDto);
      }

      return visitorDtos;
   }

   @Override
   public void saveVisitor(VisitorDto visitorDto) {
      String generatedOtp = Integer.toString((int)(Math.random() * 900000) + 100000);
      Visitor visitor = new Visitor(visitorDto.getName(), visitorDto.getPhoneNo(), visitorDto.getResidentEmail(), generatedOtp);
      visitorRepository.save(visitor);
   }

   @Override
   public String getRecentVisitorOtp(String email) {
      List<Visitor> visitors = visitorRepository.getVisitorsByEmail(email);
      Visitor visitor = visitors.get(visitors.size() - 1);
      return visitor.getOtp();
   }


   @Override
   public String getVisitorOtpById(Integer visitorId) {
      Visitor visitor = visitorRepository.getVisitorById(visitorId).get(0);
      if(visitor != null) {
         return visitor.getOtp();
      }
      return null;
   }
}