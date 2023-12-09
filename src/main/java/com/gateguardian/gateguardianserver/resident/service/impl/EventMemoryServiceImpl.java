package com.gateguardian.gateguardianserver.resident.service.impl;

import com.gateguardian.gateguardianserver.resident.dto.EventMemoryDto;
import com.gateguardian.gateguardianserver.resident.model.EventMemory;
import com.gateguardian.gateguardianserver.resident.repository.EventMemoryRepository;
import com.gateguardian.gateguardianserver.resident.service.EventMemoryService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class EventMemoryServiceImpl implements EventMemoryService {

   @Autowired
   private EventMemoryRepository eventMemoryRepository;

   @Override
   public List<EventMemory> getEventMemoriesByResident(String email) {
      return eventMemoryRepository.getResidentByEmail(email);
   }

}