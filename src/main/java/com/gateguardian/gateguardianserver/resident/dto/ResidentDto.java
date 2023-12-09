package com.gateguardian.gateguardianserver.resident.dto;

public class ResidentDto {
   private String name;
   private String email;
   private Integer flatNo;
   private String building;

   public ResidentDto() {}

   public ResidentDto(String name, String email, Integer flatNo, String building) {
      this.name = name;
      this.email = email;
      this.flatNo = flatNo;
      this.building = building;
   }

   public String getName() { return name; }
   public void setName(String name) { this.name = name; }

   public String getEmail() { return email; }
   public void setEmail(String email) { this.email = email; }

   public Integer getFlatNo() { return flatNo; }
   public void setFlatNo(Integer flatNo) { this.flatNo = flatNo; }

   public String getBuilding() { return building; }
   public void setBuilding(String building) { this.building = building; }
}