package com.gateguardian.gateguardianserver.resident.repository;

import com.gateguardian.gateguardianserver.resident.model.Visitor;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface VisitorRepository extends JpaRepository<Visitor, Integer> {

   @Query(value = "SELECT * FROM visitors WHERE host_email = :email", nativeQuery = true)
   List<Visitor> getVisitorsByEmail(@Param("email")String email);

   @Query(value = "SELECT * FROM visitors WHERE visitor_id = :visitorId", nativeQuery = true)
   List<Visitor> getVisitorById(@Param("visitorId")Integer visitorId);

   @Query(value = "SELECT * FROM visitors WHERE host_society = :society", nativeQuery = true)
   List<Visitor> getVisitorsBySociety(@Param("society") String society);
}