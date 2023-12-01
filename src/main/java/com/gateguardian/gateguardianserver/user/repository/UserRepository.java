package com.gateguardian.gateguardianserver.user.repository;

import com.gateguardian.gateguardianserver.user.model.User;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface UserRepository extends JpaRepository<User, Integer> {

   @Query(value = "SELECT * FROM users WHERE email = :email", nativeQuery = true)
   List<User> getUserByEmail(@Param("email") String email);
}