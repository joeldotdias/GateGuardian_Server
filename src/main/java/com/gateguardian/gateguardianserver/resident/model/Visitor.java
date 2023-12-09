package com.gateguardian.gateguardianserver.resident.model;

import jakarta.persistence.*;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.expression.ExpressionParser;
import org.springframework.expression.spel.standard.SpelExpressionParser;

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

   @Column(name = "resident_email")
   private String residentEmail;

   @Column(name = "otp")
   private String otp;

   public Visitor() {}

   public Visitor(String name, String phoneNo, String residentEmail, String otp) {
      this.name = name;
      this.phoneNo = phoneNo;
      this.residentEmail = residentEmail;
      this.otp = otp;
   }

   public Visitor(Integer visitorId, String name, String phoneNo, String residentEmail, String otp) {
      this.visitorId = visitorId;
      this.name = name;
      this.phoneNo = phoneNo;
      this.residentEmail = residentEmail;
      this.otp = otp;
   }

   public Integer getVisitorId() { return visitorId; }
   public void setVisitorId(Integer visitorId) { this.visitorId = visitorId; }

   public String getName() { return name; }
   public void setName(String name) { this.name = name; }

   public String getPhoneNo() { return phoneNo; }
   public void setPhoneNo(String phoneNo) { this.phoneNo = phoneNo; }

   public String getResidentEmail() { return residentEmail; }
   public void setResidentEmail(String residentEmail) { this.residentEmail = residentEmail; }

   public String getOtp() { return otp; }
   public void setOtp(String otp) {
      this.otp  = otp;
   }
}