package com.gateguardian.gateguardianserver.security.service;

import com.gateguardian.gateguardianserver.resident.model.Visitor;

import java.util.List;

public interface VisitorLogService {
   List<Visitor> getVisitors();
}