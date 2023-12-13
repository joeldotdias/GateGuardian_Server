package com.gateguardian.gateguardianserver.resident.model;

import jakarta.persistence.*;

@Entity
@Table(name = "visitors")
public class Visitor {

   @Id
   @GeneratedValue(strategy = GenerationType.IDENTITY)
   @Column(name = "visitor_id")
   private Integer visitorId;

   @Column(name = "name")
   private String name;

   @Column(name = "phone_no")
   private String phoneNo;

   @Column(name = "host_email")
   private String hostEmail;

   @Column(name = "host_flat")
   private Integer hostFlat;

   @Column(name = "host_building")
   private String hostBuilding;

   @Column(name = "host_society")
   private String society;

   @Column(name = "uid")
   private String uid;

   @Column(name = "otp")
   private String otp;

   public Visitor() {}

   public Visitor(String name, String phoneNo, String hostEmail, Integer hostFlat, String hostBuilding, String society, String uid, String otp) {
      this.name = name;
      this.phoneNo = phoneNo;
      this.hostEmail = hostEmail;
      this.hostFlat = hostFlat;
      this.hostBuilding = hostBuilding;
      this.society = society;
      this.uid = uid;
      this.otp = otp;
   }

//   public Visitor(Integer visitorId, String name, String phoneNo, String hostEmail, Integer hostFlat, String hostBuilding, String society, String uid, String otp) {
//      this.visitorId = visitorId;
//      this.name = name;
//      this.phoneNo = phoneNo;
//      this.hostEmail = hostEmail;
//      this.hostFlat = hostFlat;
//      this.hostBuilding = hostBuilding;
//      this.society = society;
//      this.uid = uid;
//      this.otp = otp;
//   }

   public Integer getVisitorId() { return visitorId; }
   public void setVisitorId(Integer visitorId) { this.visitorId = visitorId; }

   public String getName() { return name; }
   public void setName(String name) { this.name = name; }

   public String getPhoneNo() { return phoneNo; }
   public void setPhoneNo(String phoneNo) { this.phoneNo = phoneNo; }

   public String getHostEmail() { return hostEmail; }
   public void setHostEmail(String hostEmail) { this.hostEmail = hostEmail; }

   public Integer getHostFlat() { return hostFlat; }
   public void setHostFlat(Integer hostFlat) { this.hostFlat = hostFlat; }

   public String getHostBuilding() { return hostBuilding; }
   public void setHostBuilding(String hostBuilding) { this.hostBuilding = hostBuilding; }

   public String getSociety() { return society; }
   public void setSociety(String society) { this.society = society; }

   public String getUid() { return uid; }
   public void setUid(String uid) { this.uid = uid; }

   public String getOtp() { return otp; }
   public void setOtp(String otp) {
      this.otp  = otp;
   }
}