package com.gateguardian.gateguardianserver.security.repository;

import com.gateguardian.gateguardianserver.security.model.Security;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface SecurityRepository extends JpaRepository<Security, Integer> {
}