package com.gateguardian.gateguardianserver.security.model;

import jakarta.persistence.*;

import java.time.LocalDateTime;

@Entity
@Table(name = "visitor_logs")
public class VisitorLog {

   @Id
   @GeneratedValue(strategy = GenerationType.IDENTITY)
   @Column(name = "log_id")
   private Integer logId;

   @Column(name = "name")
   private String name;

   @Column(name = "phoneNo")
   private String phoneNo;

   @Column(name = "host_flat")
   private Integer hostFlat;

   @Column(name = "host_building")
   private String hostBuilding;

   @Column(name = "host_society")
   private String hostSociety;

   @Column(name = "entry")
   private LocalDateTime entry;

   public VisitorLog() {}

   public VisitorLog(String name, String phoneNo, Integer hostFlat, String hostBuilding, String hostSociety) {
      this.name = name;
      this.phoneNo = phoneNo;
      this.hostFlat = hostFlat;
      this.hostBuilding = hostBuilding;
      this.hostSociety = hostSociety;
      this.entry = LocalDateTime.now();
   }

   public Integer getLogId() { return logId; }
   public void setLogId(Integer logId) { this.logId = logId; }

   public String getName() { return name; }
   public void setName(String name) { this.name = name; }

   public String getPhoneNo() { return phoneNo; }
   public void setPhoneNo(String phoneNo) { this.phoneNo = phoneNo; }

   public Integer getHostFlat() { return hostFlat; }
   public void setHostFlat(Integer hostFlat) { this.hostFlat = hostFlat; }

   public String getHostBuilding() { return hostBuilding; }
   public void setHostBuilding(String hostBuilding) { this.hostBuilding = hostBuilding; }

   public String getHostSociety() { return hostSociety; }
   public void setHostSociety(String hostSociety) { this.hostSociety = hostSociety; }

   public LocalDateTime getEntry() { return entry; }
   public void setEntry(LocalDateTime entry) { this.entry = entry; }
}