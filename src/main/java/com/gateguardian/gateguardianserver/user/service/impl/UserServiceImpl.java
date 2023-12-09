package com.gateguardian.gateguardianserver.user.service.impl;

import com.gateguardian.gateguardianserver.resident.model.Resident;
import com.gateguardian.gateguardianserver.resident.repository.ResidentRepository;
import com.gateguardian.gateguardianserver.security.model.Security;
import com.gateguardian.gateguardianserver.security.repository.SecurityRepository;
import com.gateguardian.gateguardianserver.user.model.User;
import com.gateguardian.gateguardianserver.user.repository.UserRepository;
import com.gateguardian.gateguardianserver.user.service.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class UserServiceImpl implements UserService {

   @Autowired
   private UserRepository userRepository;

   @Autowired
   ResidentRepository residentRepository;

   @Autowired
   SecurityRepository securityRepository;

   @Override
   public User getUserByEmail(String email) {
      return userRepository.getUsersByEmail(email).get(0);
   }

   @Override
   public List<User> getUsersBySociety(String society) {
      return userRepository.getUsersBySociety(society);
   }

   @Override
   public void saveUser(User user) {
      userRepository.save(user);

      String name = user.getName();
      String email = user.getEmail();
      String society = user.getSociety();
      String userCategory = user.getCategory();

      if(userCategory.equalsIgnoreCase("resident") || userCategory.equalsIgnoreCase("admin")) {
         residentRepository.save(new Resident(name, email, society));
      }
      else if(userCategory.equalsIgnoreCase("security")) {
         securityRepository.save(new Security(name, email, society));
      }
      else {
         // Todo handle error
      }
   }
}