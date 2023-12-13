package com.gateguardian.gateguardianserver.resident.dto;

public class VisitorCredDto {
   private String uid;
   private String otp;

   public VisitorCredDto() {}

   public VisitorCredDto(String uid, String otp) {
      this.uid = uid;
      this.otp = otp;
   }

   public String getUid() { return uid; }
   public void setUid(String uid) { this.uid = uid; }

   public String getOtp() { return otp; }
   public void setOtp(String otp) { this.otp = otp; }
}