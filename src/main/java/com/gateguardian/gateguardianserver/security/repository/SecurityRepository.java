package com.gateguardian.gateguardianserver.security.repository;

import com.gateguardian.gateguardianserver.security.SecurityDto;
import com.gateguardian.gateguardianserver.security.model.Security;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface SecurityRepository extends JpaRepository<Security, Integer> {

   @Query(value = "SELECT * FROM securities where email = :email", nativeQuery = true)
   List<Security> getSecurityByEmail(@Param("email") String email);

   @Query(value = "SELECT * FROM securities where society = :society", nativeQuery = true)
   List<Security> getSecurityBySociety(@Param("society") String society);
}