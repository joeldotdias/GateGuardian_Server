package com.gateguardian.gateguardianserver.security.model;

import jakarta.persistence.*;

@Entity
@Table(name = "security")
public class Security {

   @Id
   @GeneratedValue(strategy = GenerationType.IDENTITY)
   @Column(name = "security_id")
   private Integer securityId;

   @Column(name = "name")
   private String name;

   @Column(name = "email")
   private String email;

   @Column(name = "pfp_url")
   private String pfpUrl;

   @Column(name = "phone_no")
   private String phoneNo;

   @Column(name = "society")
   private String society;

   public Security() {}

   public Security(String name, String email, String society) {
      this.name = name;
      this.email = email;
      this.society = society;
   }

   public Security(Integer securityId, String name, String email, String pfpUrl, String phoneNo, String society) {
      this.securityId = securityId;
      this.name = name;
      this.email = email;
      this.pfpUrl = pfpUrl;
      this.phoneNo = phoneNo;
      this.society = society;
   }

   public Integer getSecurityId() { return securityId; }
   public void setSecurityId(Integer securityId) { this.securityId = securityId; }

   public String getName() { return name; }
   public void setName(String name) { this.name = name; }

   public String getEmail() { return email; }
   public void setEmail(String email) { this.email = email; }

   public String getPfpUrl() { return pfpUrl; }
   public void setPfpUrl(String pfpUrl) { this.pfpUrl = pfpUrl; }

   public String getPhoneNo() { return phoneNo; }
   public void setPhoneNo(String phoneNo) { this.phoneNo = phoneNo; }

   public String getSociety() { return society; }
   public void setSociety(String society) { this.society = society; }
}