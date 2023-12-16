package com.gateguardian.gateguardianserver.security.repository;

import com.gateguardian.gateguardianserver.security.model.VisitorLog;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface VisitorLogRepository extends JpaRepository<VisitorLog, Integer> {

   @Query(value = "SELECT * FROM visitor_logs WHERE host_society = :society", nativeQuery = true)
   public List<VisitorLog> getVisitorLogsBySociety(@Param("society")String society);
}