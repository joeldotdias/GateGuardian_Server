package com.gateguardian.gateguardianserver.resident.repository;

import com.gateguardian.gateguardianserver.resident.model.EventMemory;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface EventMemoryRepository extends JpaRepository<EventMemory, Integer> {

   @Query(value = "SELECT * FROM event_memories where author_email = :email", nativeQuery = true)
   List<EventMemory> getResidentByEmail(@Param("email") String email);
}