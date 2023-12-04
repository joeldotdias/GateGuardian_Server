package com.gateguardian.gateguardianserver.resident.model;

import jakarta.persistence.*;

@Entity
@Table(name = "residents")
public class Resident {

   @Id
   @GeneratedValue(strategy = GenerationType.IDENTITY)
   @Column(name = "resident_id")
   private Integer residentId;

   @Column(name = "name")
   private String name;

   @Column(name = "email")
   private String email;

   @Column(name = "pfp_url")
   private String pfpUrl;

   @Column(name = "about_me")
   private String aboutMe;

   @Column(name = "phone_no")
   private String phoneNo;

   @Column(name = "flat_no")
   private Integer flatNo;

   @Column(name = "building")
   private String building;

   @Column(name = "society")
   private String society;

   public Resident() {}

   public Resident(Integer residentId, String name, String email, String pfpUrl, String aboutMe, String phoneNo, Integer flatNo, String building, String society) {
      this.residentId = residentId;
      this.name = name;
      this.email = email;
      this.pfpUrl = pfpUrl;
      this.aboutMe = aboutMe;
      this.phoneNo = phoneNo;
      this.flatNo = flatNo;
      this.building = building;
      this.society = society;
   }

   public Integer getResidentId() {
      return residentId;
   }

   public void setResidentId(Integer residentId) {
      this.residentId = residentId;
   }

   public String getName() { return name; }
   public void setName(String name) {
      this.name = name;
   }

   public String getEmail() {
      return email;
   }
   public void setEmail(String email) {
      this.email = email;
   }

   public String getPfpUrl() {
      return pfpUrl;
   }
   public void setPfpUrl(String pfpUrl) {
      this.pfpUrl = pfpUrl;
   }

   public String getAboutMe() { return aboutMe; }
   public void setAboutMe(String aboutMe) { this.aboutMe = aboutMe; }

   public String getPhoneNo() { return phoneNo; }
   public void setPhoneNo(String phoneNo) { this.phoneNo = phoneNo; }

   public Integer getFlatNo() {
      return flatNo;
   }
   public void setFlatNo(Integer flatNo) {
      this.flatNo = flatNo;
   }

   public String getBuilding() {
      return building;
   }
   public void setBuilding(String building) {
      this.building = building;
   }

   public String getSociety() {
      return society;
   }
   public void setSociety(String society) {
      this.society = society;
   }
}