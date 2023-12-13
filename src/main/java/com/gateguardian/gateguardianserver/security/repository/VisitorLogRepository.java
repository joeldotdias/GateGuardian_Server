package com.gateguardian.gateguardianserver.security.repository;

import com.gateguardian.gateguardianserver.security.model.VisitorLog;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface VisitorLogRepository extends JpaRepository<VisitorLog, Integer> {

}