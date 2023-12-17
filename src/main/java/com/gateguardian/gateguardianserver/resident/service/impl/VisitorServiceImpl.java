package com.gateguardian.gateguardianserver.resident.service.impl;

import com.gateguardian.gateguardianserver.resident.dto.VisitorCredDto;
import com.gateguardian.gateguardianserver.resident.dto.VisitorDto;
import com.gateguardian.gateguardianserver.resident.model.Resident;
import com.gateguardian.gateguardianserver.resident.model.Visitor;
import com.gateguardian.gateguardianserver.resident.repository.ResidentRepository;
import com.gateguardian.gateguardianserver.resident.repository.VisitorRepository;
import com.gateguardian.gateguardianserver.resident.service.VisitorService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.UUID;

@Service
public class VisitorServiceImpl implements VisitorService {

   @Autowired
   private VisitorRepository visitorRepository;

   @Autowired
   private ResidentRepository residentRepository;

   @Override
   public List<Visitor> getVisitorsByEmail(String email) {
      return visitorRepository.getVisitorsByEmail(email);
   }

   @Override
   public void saveVisitor(VisitorDto visitorDto) {
      String visitorUid = UUID.randomUUID().toString().substring(0,6).toUpperCase();
      String generatedOtp = Integer.toString((int)(Math.random() * 900000) + 100000);
      Resident resident = residentRepository.getResidentByEmail(visitorDto.getHostEmail()).get(0);
      Visitor visitor = new Visitor(visitorDto.getName(), visitorDto.getPhoneNo(), visitorDto.getHostEmail(), resident.getFlatNo(), resident.getBuilding(), resident.getSociety(), visitorUid, generatedOtp);
      visitorRepository.save(visitor);
   }

   @Override
   public String getRecentVisitorOtp(String email) {
      List<Visitor> visitors = visitorRepository.getVisitorsByEmail(email);
      Visitor visitor = visitors.get(visitors.size() - 1);
      return visitor.getOtp();
   }

   @Override
   public VisitorCredDto getRecentVisitorCred(String email) {
      List<Visitor> visitors = visitorRepository.getVisitorsByEmail(email);
      Visitor visitor = visitors.get(visitors.size() - 1);
      return new VisitorCredDto(visitor.getUid(), visitor.getOtp());
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