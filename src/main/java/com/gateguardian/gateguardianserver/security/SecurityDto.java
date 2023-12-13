package com.gateguardian.gateguardianserver.security;

public class SecurityDto {
   private String name;
   private String email;
   private String badgeId;

   public SecurityDto() {}

   public SecurityDto(String name, String email, String badgeId) {
      this.name = name;
      this.email = email;
      this.badgeId = badgeId;
   }

   public String getName() { return name; }
   public void setName(String name) { this.name = name; }

   public String getEmail() { return email; }
   public void setEmail(String email) { this.email = email; }

   public String getBadgeId() { return badgeId; }
   public void setBadgeId(String badgeId) { this.badgeId = badgeId; }
}