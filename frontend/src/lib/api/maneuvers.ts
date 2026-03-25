import { apiClient } from "../api-client";
import type {
  GetManeuverByIdRequest,
  GetManeuverByIdResponse,
  ListManeuversRequest,
  ListManeuversResponse,
} from "./models/maneuvers";

export const maneuversApi = {
  getById: async (
    req: GetManeuverByIdRequest
  ): Promise<GetManeuverByIdResponse> => {
    const { data } = await apiClient.get<GetManeuverByIdResponse>(
      `/maneuvers/${req.id}`
    );
    return data;
  },

  list: async (req: ListManeuversRequest): Promise<ListManeuversResponse> => {
    const params = new URLSearchParams();
    if (req.page !== undefined) params.append("page", req.page.toString());
    if (req.pageSize !== undefined)
      params.append("page_size", req.pageSize.toString());

    const { data } = await apiClient.get<ListManeuversResponse>("/maneuvers", {
      params,
    });
    return data;
  },
};
