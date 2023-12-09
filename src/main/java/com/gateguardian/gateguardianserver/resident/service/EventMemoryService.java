package com.gateguardian.gateguardianserver.resident.service;

import com.gateguardian.gateguardianserver.resident.dto.EventMemoryDto;
import com.gateguardian.gateguardianserver.resident.model.EventMemory;

import java.util.List;

public interface EventMemoryService {

   List<EventMemory> getEventMemoriesByResident(String email);

}