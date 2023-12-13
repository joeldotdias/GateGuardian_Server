package com.gateguardian.gateguardianserver.security.model;

import jakarta.persistence.*;

import java.time.LocalDateTime;

@Entity
@Table(name = "visitor_logs")
public class VisitorLog {

   @Id
   @GeneratedValue(strategy = GenerationType.IDENTITY)
   @Column(name = "visitor_id")
   private Integer visitorId;

   @Column(name = "name")
   private String name;

   @Column(name = "phoneNo")
   private String phoneNo;

   @Column(name = "host_flat")
   private String hostFlat;

   @Column(name = "host_building")
   private String hostBuilding;

   @Column(name = "entry")
   private LocalDateTime entry;
}