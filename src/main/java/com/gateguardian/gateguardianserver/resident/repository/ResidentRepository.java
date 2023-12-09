package com.gateguardian.gateguardianserver.resident.repository;

import com.gateguardian.gateguardianserver.resident.model.Resident;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface ResidentRepository extends JpaRepository<Resident, Integer> {

   @Query(value = "SELECT * FROM residents where email = :email", nativeQuery = true)
   List<Resident> getResidentByEmail(@Param("email") String email);

   @Query(value = "SELECT * FROM residents where society = :society", nativeQuery = true)
   List<Resident> getResidentsBySociety(@Param("society") String society);
}