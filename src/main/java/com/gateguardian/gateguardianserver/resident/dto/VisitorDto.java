package com.gateguardian.gateguardianserver.resident.dto;

public class VisitorDto {
   private Integer visitorId;
   private String name;
   private String phoneNo;
   private String hostEmail;

   public VisitorDto() {}

   public VisitorDto(Integer visitorId, String name, String phoneNo, String hostEmail) {
      this.visitorId = visitorId;
      this.name = name;
      this.phoneNo = phoneNo;
      this.hostEmail = hostEmail;
   }

   public Integer getVisitorId() { return visitorId; }
   public void setVisitorId(Integer visitorId) { this.visitorId = visitorId; }

   public String getName() { return name; }
   public void setName(String name) { this.name = name; }

   public String getPhoneNo() { return phoneNo; }
   public void setPhoneNo(String phoneNo) { this.phoneNo = phoneNo; }

   public String getHostEmail() { return hostEmail; }
   public void setHostEmail(String hostEmail) { this.hostEmail = hostEmail; }
}