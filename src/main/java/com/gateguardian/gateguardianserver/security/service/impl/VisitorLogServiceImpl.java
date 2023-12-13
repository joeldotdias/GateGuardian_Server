package com.gateguardian.gateguardianserver.security.service.impl;

import com.gateguardian.gateguardianserver.resident.model.Visitor;
import com.gateguardian.gateguardianserver.resident.repository.VisitorRepository;
import com.gateguardian.gateguardianserver.security.repository.VisitorLogRepository;
import com.gateguardian.gateguardianserver.security.service.VisitorLogService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class VisitorLogServiceImpl implements VisitorLogService {

   @Autowired
   private VisitorLogRepository visitorLogRepository;

   @Autowired
   private VisitorRepository visitorRepository;

   @Override
   public List<Visitor> getVisitors() {
      return null;
   }
}