package com.gateguardian.gateguardianserver.resident.dto;

public class EventMemoryDto {
   private String authorEmail;
   private String title;
   private String body;
   private String coverUrl;

   public EventMemoryDto() {}

   public EventMemoryDto(String authorEmail, String title, String body, String coverUrl) {
      this.authorEmail = authorEmail;
      this.title = title;
      this.body = body;
      this.coverUrl = coverUrl;
   }

   public String getAuthorEmail() {
      return authorEmail;
   }

   public void setAuthorEmail(String authorEmail) {
      this.authorEmail = authorEmail;
   }

   public String getTitle() {
      return title;
   }

   public void setTitle(String title) {
      this.title = title;
   }

   public String getBody() {
      return body;
   }

   public void setBody(String body) {
      this.body = body;
   }

   public String getCoverUrl() {
      return coverUrl;
   }

   public void setCoverUrl(String coverUrl) {
      this.coverUrl = coverUrl;
   }
}